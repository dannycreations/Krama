use bumpalo::collections::Vec as BumpVec;
use indexmap::IndexMap;
use krama_core::{
  AssignmentOperator, BinaryOperator, Error, ErrorKind, Expression,
  ExpressionKind, LiteralKind, ObjectKind, Span, TypeKind, UpdateOperator,
};
use parking_lot::RwLock;

use crate::Interpreter;

/// Represents a target that can be assigned to (L-Value).
/// This abstraction centralizes the logic for identifying and modifying
/// variables, properties, and array indices.
pub enum LValue<'ast> {
  Variable {
    name: &'ast str,
    distance: Option<usize>,
  },
  Property {
    properties: &'ast RwLock<IndexMap<&'ast str, ObjectKind<'ast>>>,
    name: &'ast str,
  },
  Index {
    elements: &'ast RwLock<BumpVec<'ast, ObjectKind<'ast>>>,
    index: i64,
    fixed_size: Option<i64>,
  },
}

impl<'ast> Interpreter<'ast> {
  /// Evaluates an assignment expression by resolving the LValue and updating its value.
  pub async fn eval_assignment_expression(
    &self,
    left: &Expression<'ast>,
    operator: AssignmentOperator,
    right: &Expression<'ast>,
    span: Span,
  ) -> Result<ObjectKind<'ast>, Error<'ast>> {
    // 1. Evaluate right side first.
    let right_val = self.eval_expression(right, None).await?;
    if right_val.is_control_signal() {
      return Ok(right_val);
    }

    // 2. Resolve the target (LValue).
    let target = self.resolve_lvalue(left, span).await?;

    // 3. Calculate final value based on operator (Assign vs Compound Assign).
    let final_val = if operator == AssignmentOperator::Assign {
      right_val
    } else {
      let left_val = self.get_lvalue_value(&target, left, span).await?;
      if left_val.is_control_signal() {
        return Ok(left_val);
      }

      let binary_op = self.assignment_to_binary_op(operator);
      left_val
        .binary_op(binary_op, &right_val, self.arena)
        .map_err(|k| k.at(span))?
    };

    // 4. Update the target and return the new value.
    self.set_lvalue_value(target, final_val.clone(), span)?;
    Ok(final_val)
  }

  /// Evaluates an update expression (++x, --x, x++, x--).
  pub async fn eval_update_expression(
    &self,
    operator: UpdateOperator,
    argument: &Expression<'ast>,
    prefix: bool,
    span: Span,
  ) -> Result<ObjectKind<'ast>, Error<'ast>> {
    // 1. Resolve target and get current value.
    let target = self.resolve_lvalue(argument, span).await?;
    let original_value = self.get_lvalue_value(&target, argument, span).await?;
    if original_value.is_control_signal() {
      return Ok(original_value);
    }

    let binary_op = match operator {
      UpdateOperator::Increment => BinaryOperator::Add,
      UpdateOperator::Decrement => BinaryOperator::Subtract,
    };

    // 2. Perform increment/decrement by 1.
    let new_value = original_value
      .binary_op(binary_op, &ObjectKind::Integer(1), self.arena)
      .map_err(|k| k.at(span))?;

    // 3. Update target and return appropriate value (pre vs post).
    self.set_lvalue_value(target, new_value.clone(), span)?;
    Ok(if prefix { new_value } else { original_value })
  }

  /// Resolves an expression into an LValue target.
  /// Handles identifiers, member access, and indexing.
  pub async fn resolve_lvalue(
    &self,
    expr: &Expression<'ast>,
    span: Span,
  ) -> Result<LValue<'ast>, Error<'ast>> {
    match &expr.kind {
      // 1. Resolve Variable target.
      ExpressionKind::Identifier(name) => Ok(LValue::Variable {
        name,
        distance: self.get_resolved_distance(expr),
      }),

      // 2. Resolve Property target (Member access).
      ExpressionKind::Member { object, property } => {
        let obj_val = self.eval_expression(object, None).await?;
        if obj_val.is_control_signal() {
          return Err(Error::new(
            ErrorKind::RuntimeError("Early return in LValue resolution".into()),
            span,
          ));
        }

        let name = if let ExpressionKind::Identifier(name) = property.kind {
          name
        } else {
          return Err(
            ErrorKind::TypeError("Invalid property for assignment".to_string())
              .at(span),
          );
        };

        match obj_val {
          ObjectKind::Object {
            properties,
            constant,
          } => {
            if constant {
              return Err(
                ErrorKind::TypeError(
                  "Cannot assign to property of constant object".into(),
                )
                .at(span),
              );
            }
            Ok(LValue::Property { properties, name })
          }
          ObjectKind::StructInstance {
            fields: properties, ..
          } => Ok(LValue::Property { properties, name }),
          _ => Err(
            ErrorKind::TypeError(format!(
              "Cannot assign to property of type {}",
              obj_val.type_name()
            ))
            .at(span),
          ),
        }
      }

      // 3. Resolve Index target (Array/Object indexing).
      ExpressionKind::Index { object, index } => {
        let obj_val = self.eval_expression(object, None).await?;
        if obj_val.is_control_signal() {
          return Err(Error::new(
            ErrorKind::RuntimeError("Early return in LValue resolution".into()),
            span,
          ));
        }
        let index_val = self.eval_expression(index, None).await?;
        if index_val.is_control_signal() {
          return Err(Error::new(
            ErrorKind::RuntimeError("Early return in LValue resolution".into()),
            span,
          ));
        }

        match obj_val {
          ObjectKind::Object {
            properties,
            constant,
          } => {
            if constant {
              return Err(
                ErrorKind::TypeError(
                  "Cannot assign to index of constant object".into(),
                )
                .at(span),
              );
            }
            let key = match index_val {
              ObjectKind::String(s) => s,
              _ => {
                return Err(
                  ErrorKind::TypeError("Object index must be a string".into())
                    .at(span),
                )
              }
            };
            Ok(LValue::Property {
              properties,
              name: key,
            })
          }
          ObjectKind::Array {
            elements,
            constant,
            kind,
            ..
          } => {
            if constant {
              return Err(
                ErrorKind::TypeError(
                  "Cannot assign to index of constant array".into(),
                )
                .at(span),
              );
            }
            let index = match index_val {
              ObjectKind::Integer(i) => i,
              _ => {
                return Err(
                  ErrorKind::TypeError("Array index must be an integer".into())
                    .at(span),
                )
              }
            };
            let fixed_size = if let TypeKind::Array {
              size: Some(LiteralKind::Integer(size)),
              ..
            } = &kind.kind
            {
              Some(*size)
            } else {
              None
            };
            Ok(LValue::Index {
              elements,
              index,
              fixed_size,
            })
          }
          _ => Err(
            ErrorKind::TypeError(format!(
              "Cannot assign to index of type {}",
              obj_val.type_name()
            ))
            .at(span),
          ),
        }
      }
      _ => Err(
        ErrorKind::TypeError("Invalid assignment target".to_string()).at(span),
      ),
    }
  }

  /// Retrieves the current value of an LValue.
  pub async fn get_lvalue_value(
    &self,
    target: &LValue<'ast>,
    expr: &Expression<'ast>,
    span: Span,
  ) -> Result<ObjectKind<'ast>, Error<'ast>> {
    match target {
      LValue::Variable { name, .. } => {
        self.eval_identifier(expr, name, span).await
      }
      LValue::Property { properties, name } => Ok(
        properties
          .read()
          .get(*name)
          .cloned()
          .unwrap_or(ObjectKind::Void),
      ),
      LValue::Index {
        elements, index, ..
      } => {
        let elements = elements.read();
        let idx = *index;
        let real_idx = if idx < 0 {
          elements.len() as i64 + idx
        } else {
          idx
        };
        Ok(if real_idx >= 0 && (real_idx as usize) < elements.len() {
          elements[real_idx as usize].clone()
        } else {
          ObjectKind::Void
        })
      }
    }
  }

  /// Updates the value of an LValue target.
  pub fn set_lvalue_value(
    &self,
    target: LValue<'ast>,
    value: ObjectKind<'ast>,
    span: Span,
  ) -> Result<(), Error<'ast>> {
    match target {
      LValue::Variable { name, distance } => {
        if let Some(distance) = distance {
          self.assign_at(distance, name, value, span)
        } else {
          let mut env = self.env_mut(span)?;
          if env.is_constant(name) {
            return Err(
              ErrorKind::TypeError(format!(
                "Cannot assign to constant '{}'",
                name
              ))
              .at(span),
            );
          }
          env.set(name, value, false, false);
          Ok(())
        }
      }
      LValue::Property { properties, name } => {
        properties.write().insert(name, value);
        Ok(())
      }
      LValue::Index {
        elements,
        index,
        fixed_size,
      } => {
        let mut elements = elements.write();
        let real_idx = if index < 0 {
          elements.len() as i64 + index
        } else {
          index
        };

        // Check bounds for fixed-size arrays.
        if let Some(size) = fixed_size {
          if real_idx < 0 || real_idx >= size {
            return Err(
              ErrorKind::TypeError(format!(
                "Index {} out of bounds for fixed array of size {}",
                index, size
              ))
              .at(span),
            );
          }
        }

        if real_idx >= 0 {
          let u_idx = real_idx as usize;
          if u_idx >= elements.len() {
            elements.resize(u_idx + 1, ObjectKind::Void);
          }
          elements[u_idx] = value;
        }
        Ok(())
      }
    }
  }

  /// Maps an assignment operator to its corresponding binary operator.
  fn assignment_to_binary_op(
    &self,
    operator: AssignmentOperator,
  ) -> BinaryOperator {
    match operator {
      AssignmentOperator::AddAssign => BinaryOperator::Add,
      AssignmentOperator::SubtractAssign => BinaryOperator::Subtract,
      AssignmentOperator::MultiplyAssign => BinaryOperator::Multiply,
      AssignmentOperator::DivideAssign => BinaryOperator::Divide,
      AssignmentOperator::ModuloAssign => BinaryOperator::Modulo,
      AssignmentOperator::BitwiseAndAssign => BinaryOperator::BitwiseAnd,
      AssignmentOperator::BitwiseOrAssign => BinaryOperator::BitwiseOr,
      AssignmentOperator::BitwiseXorAssign => BinaryOperator::BitwiseXor,
      AssignmentOperator::LeftShiftAssign => BinaryOperator::LeftShift,
      AssignmentOperator::RightShiftAssign => BinaryOperator::RightShift,
      AssignmentOperator::Assign => {
        unreachable!("Direct assignment should be handled separately")
      }
    }
  }
}
