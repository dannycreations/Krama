use std::rc::Rc;

use krama_core::{
  ast::{
    expression::{Expression, ExpressionKind},
    operator::{AssignmentOperator, BinaryOperator, UpdateOperator},
  },
  error::{Error, ErrorKind},
  object::Object,
  span::Span,
};

use crate::interpreter::Interpreter;

impl<'ast> Interpreter<'ast> {
  pub(crate) async fn eval_assignment_expression(
    &self,
    left: &Expression<'ast>,
    operator: AssignmentOperator,
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
    let resolved_right_val = self.resolve_object(right_val).await?;

    let distance = self.locals.borrow().get(&left.span).copied();

    if operator == AssignmentOperator::Assign {
      if let Some(distance) = distance {
        self.assign_at(distance, ident, Rc::new(resolved_right_val.clone()));
      } else {
        self.env_mut(span)?.set(
          ident,
          Rc::new(resolved_right_val.clone()),
          false,
        );
      }
      return Ok(resolved_right_val);
    }

    let left_val = if let Some(distance) = distance {
      self.get_at(distance, ident)
    } else {
      self.environment.borrow().get(ident)
    }
    .ok_or_else(|| Error {
      span,
      kind: ErrorKind::ReferenceError(ident.to_string()),
    })?;

    let binary_op = match operator {
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
    };

    let new_val = self
      .eval_binary_expression(
        binary_op,
        left_val.as_ref().clone(),
        resolved_right_val,
        span,
      )
      .await?;

    if let Some(distance) = distance {
      self.assign_at(distance, ident, Rc::new(new_val.clone()));
    } else {
      self
        .env_mut(span)?
        .set(ident, Rc::new(new_val.clone()), false);
    }

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

    let distance = self.locals.borrow().get(&argument.span).copied();
    let original_value = if let Some(distance) = distance {
      self.get_at(distance, ident)
    } else {
      self.environment.borrow().get(ident)
    }
    .ok_or_else(|| Error {
      span,
      kind: ErrorKind::ReferenceError(ident.to_string()),
    })?;
    let resolved_original_value =
      self.resolve_object(original_value.as_ref().clone()).await?;
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

    if let Some(distance) = distance {
      self.assign_at(distance, ident, Rc::new(new_value.clone()));
    } else {
      self
        .env_mut(span)?
        .set(ident, Rc::new(new_value.clone()), false);
    }

    if prefix {
      Ok(new_value)
    } else {
      Ok(resolved_original_value)
    }
  }
}
