use bumpalo::collections::String as BumpString;
use krama_core::{
  ast::operator::BinaryOperator, error::ErrorKind, object::Object, span::Span,
};

use crate::interpreter::Interpreter;

impl<'ast> Interpreter<'ast> {
  pub(crate) fn eval_binary_expression(
    &self,
    operator: BinaryOperator,
    left: Object<'ast>,
    right: Object<'ast>,
    span: Span<'ast>,
  ) -> Result<Object<'ast>, (ErrorKind, Span<'ast>)> {
    match (left, right) {
      (Object::Integer(left), Object::Integer(right)) => {
        self.eval_integer_binary_expression(operator, left, right, span)
      }
      (Object::Float(left), Object::Float(right)) => {
        self.eval_float_binary_expression(operator, left, right, span)
      }
      (Object::Integer(left), Object::Float(right)) => {
        self.eval_float_binary_expression(operator, left as f64, right, span)
      }
      (Object::Float(left), Object::Integer(right)) => {
        self.eval_float_binary_expression(operator, left, right as f64, span)
      }
      (Object::String(left), Object::String(right)) => {
        self.eval_string_binary_expression(operator, left, right, span)
      }
      (Object::Boolean(left), Object::Boolean(right)) => {
        self.eval_boolean_binary_expression(operator, left, right, span)
      }
      (l, r) => match operator {
        BinaryOperator::Equal => Ok(Object::Boolean(l == r)),
        BinaryOperator::NotEqual => Ok(Object::Boolean(l != r)),
        _ => Err((
          ErrorKind::TypeError(format!(
            "Unsupported types for binary operation: {:?} and {:?}",
            l, r
          )),
          span,
        )),
      },
    }
  }

  fn eval_integer_binary_expression(
    &self,
    operator: BinaryOperator,
    left: i64,
    right: i64,
    _span: Span,
  ) -> Result<Object<'ast>, (ErrorKind, Span<'ast>)> {
    match operator {
      BinaryOperator::Add => Ok(Object::Integer(left + right)),
      BinaryOperator::Subtract => Ok(Object::Integer(left - right)),
      BinaryOperator::Multiply => Ok(Object::Integer(left * right)),
      BinaryOperator::Divide => Ok(Object::Integer(left / right)),
      BinaryOperator::Modulo => Ok(Object::Integer(left % right)),
      BinaryOperator::Exponent => Ok(Object::Integer(left.pow(right as u32))),
      BinaryOperator::BitwiseAnd => Ok(Object::Integer(left & right)),
      BinaryOperator::BitwiseOr => Ok(Object::Integer(left | right)),
      BinaryOperator::BitwiseXor => Ok(Object::Integer(left ^ right)),
      BinaryOperator::LeftShift => Ok(Object::Integer(left << right)),
      BinaryOperator::RightShift => Ok(Object::Integer(left >> right)),
      BinaryOperator::Equal => Ok(Object::Boolean(left == right)),
      BinaryOperator::NotEqual => Ok(Object::Boolean(left != right)),
      BinaryOperator::GreaterThan => Ok(Object::Boolean(left > right)),
      BinaryOperator::GreaterThanOrEqual => Ok(Object::Boolean(left >= right)),
      BinaryOperator::LessThan => Ok(Object::Boolean(left < right)),
      BinaryOperator::LessThanOrEqual => Ok(Object::Boolean(left <= right)),
      BinaryOperator::LogicalAnd | BinaryOperator::LogicalOr => unreachable!(),
    }
  }

  fn eval_float_binary_expression(
    &self,
    operator: BinaryOperator,
    left: f64,
    right: f64,
    span: Span<'ast>,
  ) -> Result<Object<'ast>, (ErrorKind, Span<'ast>)> {
    match operator {
      BinaryOperator::Add => Ok(Object::Float(left + right)),
      BinaryOperator::Subtract => Ok(Object::Float(left - right)),
      BinaryOperator::Multiply => Ok(Object::Float(left * right)),
      BinaryOperator::Divide => Ok(Object::Float(left / right)),
      BinaryOperator::Modulo => Ok(Object::Float(left % right)),
      BinaryOperator::Exponent => Ok(Object::Float(left.powf(right))),
      BinaryOperator::Equal => Ok(Object::Boolean(left == right)),
      BinaryOperator::NotEqual => Ok(Object::Boolean(left != right)),
      BinaryOperator::GreaterThan => Ok(Object::Boolean(left > right)),
      BinaryOperator::GreaterThanOrEqual => Ok(Object::Boolean(left >= right)),
      BinaryOperator::LessThan => Ok(Object::Boolean(left < right)),
      BinaryOperator::LessThanOrEqual => Ok(Object::Boolean(left <= right)),
      _ => Err((
        ErrorKind::TypeError(format!(
          "Unsupported operator for floats: {:?}",
          operator
        )),
        span,
      )),
    }
  }

  fn eval_string_binary_expression(
    &self,
    operator: BinaryOperator,
    left: &str,
    right: &str,
    span: Span<'ast>,
  ) -> Result<Object<'ast>, (ErrorKind, Span<'ast>)> {
    match operator {
      BinaryOperator::Add => {
        let mut s = BumpString::from_str_in(left, self.arena);
        s.push_str(right);
        Ok(Object::String(s.into_bump_str()))
      }
      BinaryOperator::Equal => Ok(Object::Boolean(left == right)),
      BinaryOperator::NotEqual => Ok(Object::Boolean(left != right)),
      _ => Err((
        ErrorKind::TypeError(format!(
          "Unsupported operator for strings: {:?}",
          operator
        )),
        span,
      )),
    }
  }

  fn eval_boolean_binary_expression(
    &self,
    operator: BinaryOperator,
    left: bool,
    right: bool,
    span: Span<'ast>,
  ) -> Result<Object<'ast>, (ErrorKind, Span<'ast>)> {
    match operator {
      BinaryOperator::Equal => Ok(Object::Boolean(left == right)),
      BinaryOperator::NotEqual => Ok(Object::Boolean(left != right)),
      _ => Err((
        ErrorKind::TypeError(format!(
          "Unsupported operator for booleans: {:?}",
          operator
        )),
        span,
      )),
    }
  }
}
