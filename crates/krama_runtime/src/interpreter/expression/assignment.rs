use std::sync::Arc;

use indexmap::IndexMap;
use krama_core::{
  AssignmentOperator, Error, ErrorKind, Expression, ExpressionKind,
  LiteralKind, ObjectKind, Span, TypeKind, UpdateOperator,
};
use parking_lot::RwLock;

use crate::Interpreter;

/// Represents a target that can be assigned to (L-Value).
pub enum LValue {
  Variable {
    name: String,
    distance: Option<usize>,
  },
  Property {
    properties: Arc<RwLock<IndexMap<String, ObjectKind>>>,
    name: String,
  },
  Index {
    elements: Arc<RwLock<Vec<ObjectKind>>>,
    index: i64,
    fixed_size: Option<i64>,
  },
}

impl Interpreter {
  /// Evaluates an assignment expression.
  pub async fn eval_assignment_expression(
    &self,
    left: &Expression,
    operator: AssignmentOperator,
    right: &Expression,
    span: Span,
  ) -> Result<ObjectKind, Error> {
    let right_val = self.eval_expression(right, None).await?;
    if right_val.is_control_signal() {
      return Ok(right_val);
    }

    let target = self.resolve_lvalue(left, span).await?;

    let final_val = if operator == AssignmentOperator::Assign {
      right_val
    } else {
      let left_val = self.get_lvalue_value(&target, left, span).await?;
      if left_val.is_control_signal() {
        return Ok(left_val);
      }

      left_val
        .binary_op(operator.into(), &right_val)
        .map_err(|k| k.at(span))?
    };

    self.set_lvalue_value(target, final_val.clone(), span)?;
    Ok(final_val)
  }

  /// Evaluates an update expression (++x, --x, x++, x--).
  pub async fn eval_update_expression(
    &self,
    operator: UpdateOperator,
    argument: &Expression,
    prefix: bool,
    span: Span,
  ) -> Result<ObjectKind, Error> {
    let target = self.resolve_lvalue(argument, span).await?;
    let original_value = self.get_lvalue_value(&target, argument, span).await?;
    if original_value.is_control_signal() {
      return Ok(original_value);
    }

    let new_value = original_value
      .binary_op(operator.into(), &ObjectKind::Integer(1))
      .map_err(|k| k.at(span))?;

    self.set_lvalue_value(target, new_value.clone(), span)?;
    Ok(if prefix { new_value } else { original_value })
  }

  /// Resolves an expression into an LValue target.
  pub async fn resolve_lvalue(
    &self,
    expr: &Expression,
    span: Span,
  ) -> Result<LValue, Error> {
    match &expr.kind {
      ExpressionKind::Identifier(name) => Ok(LValue::Variable {
        name: name.to_string(),
        distance: self.get_resolved_distance(expr),
      }),

      ExpressionKind::Member { object, property } => {
        let obj_val = self.eval_expression(object, None).await?;
        if obj_val.is_control_signal() {
          return Err(Error::new(
            ErrorKind::RuntimeError("Early return in LValue resolution".into()),
            span,
          ));
        }

        let name = if let ExpressionKind::Identifier(name) = &property.kind {
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
            ..
          } => {
            if constant {
              return Err(
                ErrorKind::TypeError(
                  "Cannot assign to property of constant object".into(),
                )
                .at(span),
              );
            }
            Ok(LValue::Property {
              properties: properties.clone(),
              name: name.to_string(),
            })
          }
          _ => Err(
            ErrorKind::TypeError(format!(
              "Cannot assign to property of type {}",
              obj_val.type_name()
            ))
            .at(span),
          ),
        }
      }

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
            ..
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
              properties: properties.clone(),
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
            let index = self.ensure_int_index(&index_val, span)?;
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
              elements: elements.clone(),
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
    target: &LValue,
    expr: &Expression,
    span: Span,
  ) -> Result<ObjectKind, Error> {
    match target {
      LValue::Variable { name, .. } => {
        self.eval_identifier(expr, name, span).await
      }
      LValue::Property { properties, name } => Ok(
        properties
          .read()
          .get(name)
          .cloned()
          .unwrap_or(ObjectKind::Void),
      ),
      LValue::Index {
        elements, index, ..
      } => Ok(self.get_by_index(&elements.read(), *index)),
    }
  }

  /// Updates the value of an LValue target.
  pub fn set_lvalue_value(
    &self,
    target: LValue,
    value: ObjectKind,
    span: Span,
  ) -> Result<(), Error> {
    match target {
      LValue::Variable { name, distance } => {
        if let Some(distance) = distance {
          self.assign_at(distance, &name, value, span)
        } else {
          let mut env = self.env_mut(span)?;
          if env.is_constant(&name) {
            return Err(
              ErrorKind::TypeError(format!(
                "Cannot assign to constant '{}'",
                name
              ))
              .at(span),
            );
          }
          // We use get_mut here because resolve_lvalue already verified the variable exists
          // and we already checked for constant.
          if let Some(binding) = env.store.get_mut(&name) {
            binding.value = value;
            Ok(())
          } else {
            Err(Error::new(
              ErrorKind::ReferenceError(format!("Unknown variable '{}'", name)),
              span,
            ))
          }
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
        let len = elements.len();
        let real_idx = self.resolve_index(index, len);

        if let Some(size) = fixed_size {
          let check_idx = if index < 0 { size + index } else { index };
          if check_idx < 0 || check_idx >= size {
            return Err(
              ErrorKind::TypeError(format!(
                "Index {} out of bounds for fixed array of size {}",
                index, size
              ))
              .at(span),
            );
          }
        }

        if let Some(i) = real_idx {
          elements[i] = value;
        } else if index >= 0 {
          let u_idx = index as usize;
          elements.resize(u_idx + 1, ObjectKind::Void);
          elements[u_idx] = value;
        }
        Ok(())
      }
    }
  }
}
