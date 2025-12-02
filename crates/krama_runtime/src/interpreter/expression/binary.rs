use bumpalo::collections::String as BumpString;
use krama_core::{
  ast::operator::BinaryOperator,
  error::{Error, ErrorKind},
  object::Object,
  span::Span,
};

use crate::interpreter::Interpreter;

macro_rules! numeric_op {
  ($operator:expr, $left:expr, $right:expr, $span:expr, $op:tt, $return_type:ident) => {
      Ok(Object::$return_type($left $op $right))
  };
}

impl<'ast> Interpreter<'ast> {
  pub(crate) fn eval_binary_expression(
    &self,
    operator: BinaryOperator,
    left: Object<'ast>,
    right: Object<'ast>,
    span: Span<'ast>,
  ) -> Result<Object<'ast>, Error<'ast>> {
    match (left, right) {
      (Object::Integer(l), Object::Integer(r)) => {
        self.eval_integer_binary_expression(operator, l, r, span)
      }
      (Object::Float(l), Object::Float(r)) => {
        self.eval_float_binary_expression(operator, l, r, span)
      }
      (Object::Integer(l), Object::Float(r)) => {
        self.eval_float_binary_expression(operator, l as f64, r, span)
      }
      (Object::Float(l), Object::Integer(r)) => {
        self.eval_float_binary_expression(operator, l, r as f64, span)
      }
      (Object::String(l), Object::String(r)) => {
        self.eval_string_binary_expression(operator, l, r, span)
      }
      (Object::Boolean(l), Object::Boolean(r)) => {
        self.eval_boolean_binary_expression(operator, l, r, span)
      }
      (l, r) => match operator {
        BinaryOperator::Equal => Ok(Object::Boolean(l == r)),
        BinaryOperator::NotEqual => Ok(Object::Boolean(l != r)),
        _ => Err(Error::new(
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
    span: Span<'ast>,
  ) -> Result<Object<'ast>, Error<'ast>> {
    match operator {
      BinaryOperator::Add => {
        numeric_op!(operator, left, right, span, +, Integer)
      }
      BinaryOperator::Subtract => {
        numeric_op!(operator, left, right, span, -, Integer)
      }
      BinaryOperator::Multiply => {
        numeric_op!(operator, left, right, span, *, Integer)
      }
      BinaryOperator::Divide => {
        numeric_op!(operator, left, right, span, /, Integer)
      }
      BinaryOperator::Modulo => {
        numeric_op!(operator, left, right, span, %, Integer)
      }
      BinaryOperator::Exponent => Ok(Object::Integer(left.pow(right as u32))),
      BinaryOperator::BitwiseAnd => {
        numeric_op!(operator, left, right, span, &, Integer)
      }
      BinaryOperator::BitwiseOr => {
        numeric_op!(operator, left, right, span, |, Integer)
      }
      BinaryOperator::BitwiseXor => {
        numeric_op!(operator, left, right, span, ^, Integer)
      }
      BinaryOperator::LeftShift => {
        numeric_op!(operator, left, right, span, <<, Integer)
      }
      BinaryOperator::RightShift => {
        numeric_op!(operator, left, right, span, >>, Integer)
      }
      BinaryOperator::Equal => {
        numeric_op!(operator, left, right, span, ==, Boolean)
      }
      BinaryOperator::NotEqual => {
        numeric_op!(operator, left, right, span, !=, Boolean)
      }
      BinaryOperator::GreaterThan => {
        numeric_op!(operator, left, right, span, >, Boolean)
      }
      BinaryOperator::GreaterThanOrEqual => {
        numeric_op!(operator, left, right, span, >=, Boolean)
      }
      BinaryOperator::LessThan => {
        numeric_op!(operator, left, right, span, <, Boolean)
      }
      BinaryOperator::LessThanOrEqual => {
        numeric_op!(operator, left, right, span, <=, Boolean)
      }
      BinaryOperator::LogicalAnd | BinaryOperator::LogicalOr => {
        Err(Error::new(
          ErrorKind::TypeError(format!(
            "Unsupported operator for integers: {:?}",
            operator
          )),
          span,
        ))
      }
    }
  }

  fn eval_float_binary_expression(
    &self,
    operator: BinaryOperator,
    left: f64,
    right: f64,
    span: Span<'ast>,
  ) -> Result<Object<'ast>, Error<'ast>> {
    match operator {
      BinaryOperator::Add => numeric_op!(operator, left, right, span, +, Float),
      BinaryOperator::Subtract => {
        numeric_op!(operator, left, right, span, -, Float)
      }
      BinaryOperator::Multiply => {
        numeric_op!(operator, left, right, span, *, Float)
      }
      BinaryOperator::Divide => {
        numeric_op!(operator, left, right, span, /, Float)
      }
      BinaryOperator::Modulo => {
        numeric_op!(operator, left, right, span, %, Float)
      }
      BinaryOperator::Exponent => Ok(Object::Float(left.powf(right))),
      BinaryOperator::Equal => {
        numeric_op!(operator, left, right, span, ==, Boolean)
      }
      BinaryOperator::NotEqual => {
        numeric_op!(operator, left, right, span, !=, Boolean)
      }
      BinaryOperator::GreaterThan => {
        numeric_op!(operator, left, right, span, >, Boolean)
      }
      BinaryOperator::GreaterThanOrEqual => {
        numeric_op!(operator, left, right, span, >=, Boolean)
      }
      BinaryOperator::LessThan => {
        numeric_op!(operator, left, right, span, <, Boolean)
      }
      BinaryOperator::LessThanOrEqual => {
        numeric_op!(operator, left, right, span, <=, Boolean)
      }
      _ => Err(Error::new(
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
  ) -> Result<Object<'ast>, Error<'ast>> {
    match operator {
      BinaryOperator::Add => {
        let mut s = BumpString::from_str_in(left, self.arena);
        s.push_str(right);
        Ok(Object::String(s.into_bump_str()))
      }
      BinaryOperator::Equal => Ok(Object::Boolean(left == right)),
      BinaryOperator::NotEqual => Ok(Object::Boolean(left != right)),
      _ => Err(Error::new(
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
  ) -> Result<Object<'ast>, Error<'ast>> {
    match operator {
      BinaryOperator::Equal => Ok(Object::Boolean(left == right)),
      BinaryOperator::NotEqual => Ok(Object::Boolean(left != right)),
      _ => Err(Error::new(
        ErrorKind::TypeError(format!(
          "Unsupported operator for booleans: {:?}",
          operator
        )),
        span,
      )),
    }
  }
}
