use bumpalo::collections::{String as BumpString, Vec as BumpVec};
use krama_core::{BinaryOperator, Error, ErrorKind, Object, Span};

use crate::Interpreter;

impl<'ast> Interpreter<'ast> {
  pub fn eval_binary_expression(
    &self,
    operator: BinaryOperator,
    left: Object<'ast>,
    right: Object<'ast>,
    span: Span<'ast>,
  ) -> Result<Object<'ast>, Error<'ast>> {
    match (left, right) {
      (Object::Integer(l), Object::Integer(r)) => match operator {
        BinaryOperator::Add => Ok(Object::Integer(l + r)),
        BinaryOperator::Subtract => Ok(Object::Integer(l - r)),
        BinaryOperator::Multiply => Ok(Object::Integer(l * r)),
        BinaryOperator::Divide => Ok(Object::Integer(l / r)),
        BinaryOperator::Modulo => Ok(Object::Integer(l % r)),
        BinaryOperator::Exponent => Ok(Object::Integer(l.pow(r as u32))),
        BinaryOperator::BitwiseAnd => Ok(Object::Integer(l & r)),
        BinaryOperator::BitwiseOr => Ok(Object::Integer(l | r)),
        BinaryOperator::BitwiseXor => Ok(Object::Integer(l ^ r)),
        BinaryOperator::LeftShift => Ok(Object::Integer(l << r)),
        BinaryOperator::RightShift => Ok(Object::Integer(l >> r)),
        BinaryOperator::Equal => Ok(Object::Boolean(l == r)),
        BinaryOperator::NotEqual => Ok(Object::Boolean(l != r)),
        BinaryOperator::GreaterThan => Ok(Object::Boolean(l > r)),
        BinaryOperator::GreaterThanOrEqual => Ok(Object::Boolean(l >= r)),
        BinaryOperator::LessThan => Ok(Object::Boolean(l < r)),
        BinaryOperator::LessThanOrEqual => Ok(Object::Boolean(l <= r)),
        BinaryOperator::Range => {
          let mut elements = BumpVec::new_in(self.arena);
          for i in l..=r {
            elements.push(Object::Integer(i));
          }
          Ok(Object::Tuple {
            elements: elements.into_bump_slice(),
          })
        }
        _ => Err(Error::new(
          ErrorKind::TypeError(format!(
            "Unsupported operator for integers: {:?}",
            operator
          )),
          span,
        )),
      },
      (Object::Float(l), Object::Float(r)) => {
        self.eval_float_op(operator, l, r, span)
      }
      (Object::Integer(l), Object::Float(r)) => {
        self.eval_float_op(operator, l as f64, r, span)
      }
      (Object::Float(l), Object::Integer(r)) => {
        self.eval_float_op(operator, l, r as f64, span)
      }
      (Object::String(l), Object::String(r)) => {
        self.eval_string_binary_expression(operator, l, r, span)
      }
      (Object::String(l), r) if operator == BinaryOperator::Add => {
        let mut s = BumpString::from_str_in(l, self.arena);
        s.push_str(&format!("{}", r));
        Ok(Object::String(s.into_bump_str()))
      }
      (l, Object::String(r)) if operator == BinaryOperator::Add => {
        let mut s = BumpString::from_str_in(&format!("{}", l), self.arena);
        s.push_str(r);
        Ok(Object::String(s.into_bump_str()))
      }
      (Object::Boolean(l), Object::Boolean(r)) => {
        self.eval_boolean_binary_expression(operator, l, r, span)
      }
      (Object::String(l), Object::Object { properties, .. }) => {
        if operator == BinaryOperator::In {
          Ok(Object::Boolean(properties.read().contains_key(l)))
        } else {
          Err(Error::new(
            ErrorKind::TypeError(format!(
              "Unsupported types for binary operation: String and Object with {:?}",
              operator
            )),
            span,
          ))
        }
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

  fn eval_float_op(
    &self,
    operator: BinaryOperator,
    left: f64,
    right: f64,
    span: Span<'ast>,
  ) -> Result<Object<'ast>, Error<'ast>> {
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
