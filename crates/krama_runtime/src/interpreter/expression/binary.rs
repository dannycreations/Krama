use crate::interpreter::Interpreter;
use bumpalo::collections::String as BumpString;
use krama_core::{
  ast::operator::BinaryOperator,
  error::{Error, ErrorKind},
  object::Object,
  span::Span,
};
use std::rc::Rc;

impl<'ast> Interpreter<'ast> {
  pub(crate) async fn eval_binary_expression(
    &self,
    operator: BinaryOperator,
    left: Object<'ast>,
    right: Object<'ast>,
    span: Span,
  ) -> Result<Object<'ast>, Error> {
    let left = self.resolve_object(left).await?;
    let right = self.resolve_object(right).await?;
    match operator {
      BinaryOperator::LogicalAnd => {
        return Ok(Object::Boolean(
          self.is_truthy(&left) && self.is_truthy(&right),
        ));
      }
      BinaryOperator::LogicalOr => {
        return Ok(Object::Boolean(
          self.is_truthy(&left) || self.is_truthy(&right),
        ));
      }
      _ => {}
    }
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
          kind: ErrorKind::InvalidOperator(format!(
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
          kind: ErrorKind::InvalidOperator(format!(
            "Unsupported operator for booleans: {:?}",
            operator
          )),
        }),
      },
      (Object::Module(left), Object::Module(right)) => match operator {
        BinaryOperator::Equal => Ok(Object::Boolean(Rc::ptr_eq(&left, &right))),
        BinaryOperator::NotEqual => {
          Ok(Object::Boolean(!Rc::ptr_eq(&left, &right)))
        }
        _ => Err(Error {
          span,
          kind: ErrorKind::InvalidOperator(format!(
            "Unsupported operator for modules: {:?}",
            operator
          )),
        }),
      },
      (l, r) => Err(Error {
        span,
        kind: ErrorKind::TypeMismatch(format!(
          "Unsupported types for binary operation: {:?} and {:?}",
          l, r
        )),
      }),
    }
  }

  fn eval_integer_binary_expression(
    &self,
    operator: BinaryOperator,
    left: i64,
    right: i64,
    span: Span,
  ) -> Result<Object<'ast>, Error> {
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
      BinaryOperator::Assign => Err(Error {
        span,
        kind: ErrorKind::InvalidOperator(
          "Assign cannot be used in a binary expression".to_string(),
        ),
      }),
      BinaryOperator::Equal => Ok(Object::Boolean(left == right)),
      BinaryOperator::NotEqual => Ok(Object::Boolean(left != right)),
      BinaryOperator::GreaterThan => Ok(Object::Boolean(left > right)),
      BinaryOperator::GreaterThanOrEqual => Ok(Object::Boolean(left >= right)),
      BinaryOperator::LessThan => Ok(Object::Boolean(left < right)),
      BinaryOperator::LessThanOrEqual => Ok(Object::Boolean(left <= right)),
      BinaryOperator::LogicalAnd | BinaryOperator::LogicalOr => {
        unreachable!()
      }
    }
  }

  fn eval_float_binary_expression(
    &self,
    operator: BinaryOperator,
    left: f64,
    right: f64,
    span: Span,
  ) -> Result<Object<'ast>, Error> {
    match operator {
      BinaryOperator::Add => Ok(Object::Float(left + right)),
      BinaryOperator::Subtract => Ok(Object::Float(left - right)),
      BinaryOperator::Multiply => Ok(Object::Float(left * right)),
      BinaryOperator::Divide => Ok(Object::Float(left / right)),
      BinaryOperator::Modulo => Ok(Object::Float(left % right)),
      BinaryOperator::Exponent => Ok(Object::Float(left.powf(right))),
      BinaryOperator::Assign => Err(Error {
        span,
        kind: ErrorKind::InvalidOperator(
          "Assign cannot be used in a binary expression".to_string(),
        ),
      }),
      BinaryOperator::Equal => Ok(Object::Boolean(left == right)),
      BinaryOperator::NotEqual => Ok(Object::Boolean(left != right)),
      BinaryOperator::GreaterThan => Ok(Object::Boolean(left > right)),
      BinaryOperator::GreaterThanOrEqual => Ok(Object::Boolean(left >= right)),
      BinaryOperator::LessThan => Ok(Object::Boolean(left < right)),
      BinaryOperator::LessThanOrEqual => Ok(Object::Boolean(left <= right)),
      BinaryOperator::LogicalAnd | BinaryOperator::LogicalOr => {
        unreachable!()
      }
      _ => Err(Error {
        span,
        kind: ErrorKind::InvalidOperator(format!(
          "Unsupported operator for floats: {:?}",
          operator
        )),
      }),
    }
  }
}
