use krama_core::ast::expression::{Expression, ExpressionKind};
use krama_core::ast::operator::{BinaryOperator, UpdateOperator};
use krama_core::error::{Error, ErrorKind};
use krama_core::object::Object;
use krama_core::span::Span;

use crate::interpreter::Interpreter;

impl<'ast> Interpreter<'ast> {
  pub(crate) async fn eval_assignment_expression(
    &self,
    left: &Expression<'ast>,
    operator: BinaryOperator,
    right: &Expression<'ast>,
    span: Span,
  ) -> Result<Object<'ast>, Error> {
    let ident = if let ExpressionKind::Identifier(name) = left.kind {
      name
    } else {
      return Err(Error {
        span,
        kind: ErrorKind::TypeError(
          "Expected identifier for assignment".to_string(),
        ),
      });
    };

    let right_val = self.eval_expression(right, None).await?;
    if operator == BinaryOperator::Assign {
      let right_val = self.resolve_object(right_val).await?;
      self
        .environment
        .borrow_mut()
        .set(ident, right_val.clone(), false);
      return Ok(right_val);
    }
    let right_val = self.resolve_object(right_val).await?;

    let left_val =
      self.environment.borrow().get(ident).ok_or_else(|| Error {
        span,
        kind: ErrorKind::ReferenceError(ident.to_string()),
      })?;
    let new_val = self
      .eval_binary_expression(operator, left_val.clone(), right_val, span)
      .await?;
    self
      .environment
      .borrow_mut()
      .set(ident, new_val.clone(), false);
    Ok(new_val)
  }

  pub(crate) async fn eval_update_expression(
    &self,
    operator: UpdateOperator,
    argument: &Expression<'ast>,
    prefix: bool,
    span: Span,
  ) -> Result<Object<'ast>, Error> {
    let ident = if let ExpressionKind::Identifier(name) = argument.kind {
      name
    } else {
      return Err(Error {
        span,
        kind: ErrorKind::TypeError(
          "Expected identifier for update expression".to_string(),
        ),
      });
    };

    let original_value =
      self.environment.borrow().get(ident).ok_or_else(|| Error {
        span,
        kind: ErrorKind::ReferenceError(ident.to_string()),
      })?;
    let resolved_original_value =
      self.resolve_object(original_value.clone()).await?;
    let new_value = match (operator, resolved_original_value.clone()) {
      (UpdateOperator::Increment, Object::Integer(i)) => Object::Integer(i + 1),
      (UpdateOperator::Decrement, Object::Integer(i)) => Object::Integer(i - 1),
      (UpdateOperator::Increment, Object::Float(f)) => Object::Float(f + 1.0),
      (UpdateOperator::Decrement, Object::Float(f)) => Object::Float(f - 1.0),
      _ => {
        return Err(Error {
          span,
          kind: ErrorKind::TypeError(
            "Update operator can only be applied to numbers".to_string(),
          ),
        })
      }
    };

    self
      .environment
      .borrow_mut()
      .set(ident, new_value.clone(), false);

    if prefix {
      Ok(new_value)
    } else {
      Ok(resolved_original_value)
    }
  }
}
