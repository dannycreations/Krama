use std::ptr;

use bumpalo::collections::String as BumpString;
use krama_core::{
  ast::operator::BinaryOperator,
  error::{Error, ErrorKind},
  object::Object,
  span::Span,
};

use crate::interpreter::Interpreter;

macro_rules! gen_numeric_binary_expr {
  ($operator:ident, $left:ident, $right:ident, $span:ident, Int) => {
    match $operator {
      BinaryOperator::Add => Ok(Object::Integer($left + $right)),
      BinaryOperator::Subtract => Ok(Object::Integer($left - $right)),
      BinaryOperator::Multiply => Ok(Object::Integer($left * $right)),
      BinaryOperator::Divide => Ok(Object::Integer($left / $right)),
      BinaryOperator::Modulo => Ok(Object::Integer($left % $right)),
      BinaryOperator::Exponent => Ok(Object::Integer($left.pow($right as u32))),
      BinaryOperator::BitwiseAnd => Ok(Object::Integer($left & $right)),
      BinaryOperator::BitwiseOr => Ok(Object::Integer($left | $right)),
      BinaryOperator::BitwiseXor => Ok(Object::Integer($left ^ $right)),
      BinaryOperator::LeftShift => Ok(Object::Integer($left << $right)),
      BinaryOperator::RightShift => Ok(Object::Integer($left >> $right)),
      BinaryOperator::Equal => Ok(Object::Boolean($left == $right)),
      BinaryOperator::NotEqual => Ok(Object::Boolean($left != $right)),
      BinaryOperator::GreaterThan => Ok(Object::Boolean($left > $right)),
      BinaryOperator::GreaterThanOrEqual => {
        Ok(Object::Boolean($left >= $right))
      }
      BinaryOperator::LessThan => Ok(Object::Boolean($left < $right)),
      BinaryOperator::LessThanOrEqual => Ok(Object::Boolean($left <= $right)),
      BinaryOperator::LogicalAnd | BinaryOperator::LogicalOr => {
        unreachable!()
      }
    }
  };
  ($operator:ident, $left:ident, $right:ident, $span:ident, Float) => {
    match $operator {
      BinaryOperator::Add => Ok(Object::Float($left + $right)),
      BinaryOperator::Subtract => Ok(Object::Float($left - $right)),
      BinaryOperator::Multiply => Ok(Object::Float($left * $right)),
      BinaryOperator::Divide => Ok(Object::Float($left / $right)),
      BinaryOperator::Modulo => Ok(Object::Float($left % $right)),
      BinaryOperator::Exponent => Ok(Object::Float($left.powf($right))),
      BinaryOperator::Equal => Ok(Object::Boolean($left == $right)),
      BinaryOperator::NotEqual => Ok(Object::Boolean($left != $right)),
      BinaryOperator::GreaterThan => Ok(Object::Boolean($left > $right)),
      BinaryOperator::GreaterThanOrEqual => {
        Ok(Object::Boolean($left >= $right))
      }
      BinaryOperator::LessThan => Ok(Object::Boolean($left < $right)),
      BinaryOperator::LessThanOrEqual => Ok(Object::Boolean($left <= $right)),
      _ => Err(Error {
        span: $span,
        kind: ErrorKind::TypeError(format!(
          "Unsupported operator for floats: {:?}",
          $operator
        )),
      }),
    }
  };
}

impl<'ast> Interpreter<'ast> {
  pub(crate) fn eval_binary_expression(
    &self,
    operator: BinaryOperator,
    left: Object<'ast>,
    right: Object<'ast>,
    span: Span,
  ) -> Result<Object<'ast>, Error> {
    match (left, right) {
      (Object::Integer(left), Object::Integer(right)) => {
        gen_numeric_binary_expr!(operator, left, right, span, Int)
      }
      (Object::Float(left), Object::Float(right)) => {
        gen_numeric_binary_expr!(operator, left, right, span, Float)
      }
      (Object::Integer(left), Object::Float(right)) => {
        let left = left as f64;
        gen_numeric_binary_expr!(operator, left, right, span, Float)
      }
      (Object::Float(left), Object::Integer(right)) => {
        let right = right as f64;
        gen_numeric_binary_expr!(operator, left, right, span, Float)
      }
      (Object::String(left), Object::String(right)) => match operator {
        BinaryOperator::Add => {
          let mut s = BumpString::from_str_in(left, self.arena);
          s.push_str(right);
          Ok(Object::String(s.into_bump_str()))
        }
        BinaryOperator::Equal => Ok(Object::Boolean(left == right)),
        BinaryOperator::NotEqual => Ok(Object::Boolean(left != right)),
        _ => Err(Error {
          span,
          kind: ErrorKind::TypeError(format!(
            "Unsupported operator for strings: {:?}",
            operator
          )),
        }),
      },
      (Object::Boolean(left), Object::Boolean(right)) => match operator {
        BinaryOperator::Equal => Ok(Object::Boolean(left == right)),
        BinaryOperator::NotEqual => Ok(Object::Boolean(left != right)),
        _ => Err(Error {
          span,
          kind: ErrorKind::TypeError(format!(
            "Unsupported operator for booleans: {:?}",
            operator
          )),
        }),
      },
      (Object::Scope(left), Object::Scope(right)) => match operator {
        BinaryOperator::Equal => Ok(Object::Boolean(ptr::eq(left, right))),
        BinaryOperator::NotEqual => Ok(Object::Boolean(!ptr::eq(left, right))),
        _ => Err(Error {
          span,
          kind: ErrorKind::TypeError(format!(
            "Unsupported operator for modules: {:?}",
            operator
          )),
        }),
      },
      (l, r) => Err(Error {
        span,
        kind: ErrorKind::TypeError(format!(
          "Unsupported types for binary operation: {:?} and {:?}",
          l, r
        )),
      }),
    }
  }
}
