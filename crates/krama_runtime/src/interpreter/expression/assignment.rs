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
    span: Span<'ast>,
  ) -> Result<Object<'ast>, Error<'ast>> {
    let ident = if let ExpressionKind::Identifier(name) = left.kind {
      name
    } else {
      return Err(Error::new(
        ErrorKind::TypeError("Expected identifier for assignment".to_string()),
        span,
      ));
    };

    let right_val = self.eval_expression(right, None).await?;

    let distance = self.get_resolved_distance(left);

    if operator == AssignmentOperator::Assign {
      if let Some(distance) = distance {
        self.assign_at(distance, ident, right_val.clone());
      } else {
        self.env_mut(span)?.set(ident, right_val.clone(), false);
      }
      return Ok(right_val);
    }

    let left_val = self.eval_identifier(left, ident, span.clone()).await?;

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

    let new_val = self.eval_binary_expression(
      binary_op,
      left_val.clone(),
      right_val,
      span.clone(),
    )?;

    if let Some(distance) = distance {
      self.assign_at(distance, ident, new_val.clone());
    } else {
      self.env_mut(span)?.set(ident, new_val.clone(), false);
    }

    Ok(new_val)
  }

  pub(crate) async fn eval_update_expression(
    &self,
    operator: UpdateOperator,
    argument: &Expression<'ast>,
    prefix: bool,
    span: Span<'ast>,
  ) -> Result<Object<'ast>, Error<'ast>> {
    let ident = if let ExpressionKind::Identifier(name) = argument.kind {
      name
    } else {
      return Err(Error::new(
        ErrorKind::TypeError(
          "Expected identifier for update expression".to_string(),
        ),
        span,
      ));
    };

    let distance = self.get_resolved_distance(argument);
    let original_value =
      self.eval_identifier(argument, ident, span.clone()).await?;

    let new_value = match (operator, &original_value) {
      (UpdateOperator::Increment, Object::Integer(i)) => Object::Integer(i + 1),
      (UpdateOperator::Decrement, Object::Integer(i)) => Object::Integer(i - 1),
      (UpdateOperator::Increment, Object::Float(f)) => Object::Float(f + 1.0),
      (UpdateOperator::Decrement, Object::Float(f)) => Object::Float(f - 1.0),
      _ => {
        return Err(Error::new(
          ErrorKind::TypeError(
            "Update operator can only be applied to numbers".to_string(),
          ),
          span,
        ))
      }
    };

    if let Some(distance) = distance {
      self.assign_at(distance, ident, new_value.clone());
    } else {
      self.env_mut(span)?.set(ident, new_value.clone(), false);
    }

    if prefix {
      Ok(new_value)
    } else {
      Ok(original_value)
    }
  }
}
