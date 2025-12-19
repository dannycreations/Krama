use krama_core::{
  AssignmentOperator, BinaryOperator, Error, ErrorKind, Expression,
  ExpressionKind, Object, Span, UpdateOperator,
};

use crate::interpreter::Interpreter;

impl<'ast> Interpreter<'ast> {
  pub async fn eval_assignment_expression(
    &self,
    left: &Expression<'ast>,
    operator: AssignmentOperator,
    right: &Expression<'ast>,
    span: Span<'ast>,
  ) -> Result<Object<'ast>, Error<'ast>> {
    let right_val = self.eval_expression(right, None).await?;

    match &left.kind {
      ExpressionKind::Identifier(ident) => {
        let distance = self.get_resolved_distance(left);

        let final_val = if operator == AssignmentOperator::Assign {
          right_val
        } else {
          let left_val =
            self.eval_identifier(left, ident, span.clone()).await?;
          let binary_op = self.assignment_to_binary_op(operator);
          self.eval_binary_expression(
            binary_op,
            left_val,
            right_val,
            span.clone(),
          )?
        };

        if let Some(distance) = distance {
          self.assign_at(distance, ident, final_val.clone());
        } else {
          self.env_mut(span)?.set(ident, final_val.clone(), false);
        }
        Ok(final_val)
      }
      ExpressionKind::Member { object, property } => {
        let obj_val = self.eval_expression(object, None).await?;
        let property_name =
          if let ExpressionKind::Identifier(name) = property.kind {
            name
          } else {
            return Err(Error::new(
              ErrorKind::TypeError("Invalid member for assignment".to_string()),
              span,
            ));
          };

        match obj_val {
          Object::Object(map) => {
            let final_val = if operator == AssignmentOperator::Assign {
              right_val
            } else {
              let left_val = map
                .read()
                .await
                .get(property_name)
                .cloned()
                .unwrap_or(Object::Void);
              let binary_op = self.assignment_to_binary_op(operator);
              self.eval_binary_expression(
                binary_op,
                left_val,
                right_val,
                span.clone(),
              )?
            };

            map.write().await.insert(property_name, final_val.clone());
            Ok(final_val)
          }
          _ => Err(Error::new(
            ErrorKind::TypeError(format!(
              "Cannot assign to property of type {}",
              obj_val.type_name()
            )),
            span,
          )),
        }
      }
      ExpressionKind::Index { object, index } => {
        let (obj_val, index_val) = futures::try_join!(
          self.eval_expression(object, None),
          self.eval_expression(index, None)
        )?;

        match obj_val {
          Object::Object(map) => {
            let key = match index_val {
              Object::String(s) => s,
              _ => {
                return Err(Error::new(
                  ErrorKind::TypeError(
                    "Object index must be a string".to_string(),
                  ),
                  span,
                ))
              }
            };

            let final_val = if operator == AssignmentOperator::Assign {
              right_val
            } else {
              let left_val =
                map.read().await.get(key).cloned().unwrap_or(Object::Void);
              let binary_op = self.assignment_to_binary_op(operator);
              self.eval_binary_expression(
                binary_op,
                left_val,
                right_val,
                span.clone(),
              )?
            };

            map.write().await.insert(key, final_val.clone());
            Ok(final_val)
          }
          _ => Err(Error::new(
            ErrorKind::TypeError(format!(
              "Cannot assign to index of type {}",
              obj_val.type_name()
            )),
            span,
          )),
        }
      }
      _ => Err(Error::new(
        ErrorKind::TypeError("Invalid assignment target".to_string()),
        span,
      )),
    }
  }

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
      AssignmentOperator::Assign => unreachable!(),
    }
  }

  pub async fn eval_update_expression(
    &self,
    operator: UpdateOperator,
    argument: &Expression<'ast>,
    prefix: bool,
    span: Span<'ast>,
  ) -> Result<Object<'ast>, Error<'ast>> {
    match &argument.kind {
      ExpressionKind::Identifier(ident) => {
        let distance = self.get_resolved_distance(argument);
        let original_value =
          self.eval_identifier(argument, ident, span.clone()).await?;

        let new_value =
          self.apply_update(operator, &original_value, span.clone())?;

        if let Some(distance) = distance {
          self.assign_at(distance, ident, new_value.clone());
        } else {
          self.env_mut(span)?.set(ident, new_value.clone(), false);
        }

        Ok(if prefix { new_value } else { original_value })
      }
      ExpressionKind::Member { object, property } => {
        let obj_val = self.eval_expression(object, None).await?;
        let property_name =
          if let ExpressionKind::Identifier(name) = property.kind {
            name
          } else {
            return Err(Error::new(
              ErrorKind::TypeError("Invalid member for update".to_string()),
              span,
            ));
          };

        match obj_val {
          Object::Object(map) => {
            let original_value = map
              .read()
              .await
              .get(property_name)
              .cloned()
              .unwrap_or(Object::Void);
            let new_value =
              self.apply_update(operator, &original_value, span.clone())?;
            map.write().await.insert(property_name, new_value.clone());
            Ok(if prefix { new_value } else { original_value })
          }
          _ => Err(Error::new(
            ErrorKind::TypeError(
              "Cannot update property of non-object".to_string(),
            ),
            span,
          )),
        }
      }
      ExpressionKind::Index { object, index } => {
        let (obj_val, index_val) = futures::try_join!(
          self.eval_expression(object, None),
          self.eval_expression(index, None)
        )?;

        match obj_val {
          Object::Object(map) => {
            let key = match index_val {
              Object::String(s) => s,
              _ => {
                return Err(Error::new(
                  ErrorKind::TypeError(
                    "Object index must be a string".to_string(),
                  ),
                  span,
                ))
              }
            };

            let original_value =
              map.read().await.get(key).cloned().unwrap_or(Object::Void);
            let new_value =
              self.apply_update(operator, &original_value, span.clone())?;
            map.write().await.insert(key, new_value.clone());
            Ok(if prefix { new_value } else { original_value })
          }
          _ => Err(Error::new(
            ErrorKind::TypeError(
              "Cannot update index of non-object".to_string(),
            ),
            span,
          )),
        }
      }
      _ => Err(Error::new(
        ErrorKind::TypeError("Invalid update target".to_string()),
        span,
      )),
    }
  }

  fn apply_update(
    &self,
    operator: UpdateOperator,
    value: &Object<'ast>,
    span: Span<'ast>,
  ) -> Result<Object<'ast>, Error<'ast>> {
    match (operator, value) {
      (UpdateOperator::Increment, Object::Integer(i)) => {
        Ok(Object::Integer(i + 1))
      }
      (UpdateOperator::Decrement, Object::Integer(i)) => {
        Ok(Object::Integer(i - 1))
      }
      (UpdateOperator::Increment, Object::Float(f)) => {
        Ok(Object::Float(f + 1.0))
      }
      (UpdateOperator::Decrement, Object::Float(f)) => {
        Ok(Object::Float(f - 1.0))
      }
      _ => Err(Error::new(
        ErrorKind::TypeError(
          "Update operator can only be applied to numbers".to_string(),
        ),
        span,
      )),
    }
  }
}
