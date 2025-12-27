use std::ops::{BitAnd, BitOr, BitXor, Shl, Shr};

use super::ObjectKind;
use crate::{BinaryOperator, ErrorKind, ErrorKindResult};

impl ObjectKind {
  pub fn binary_op(
    &self,
    operator: BinaryOperator,
    other: &Self,
  ) -> ErrorKindResult<Self> {
    if self.is_control_signal() {
      return Ok(self.clone());
    }
    if other.is_control_signal() {
      return Ok(other.clone());
    }

    match (self, other) {
      (Self::Integer(l), Self::Integer(r)) => {
        self.perform_int_op(operator, *l, *r)
      }
      (Self::Float(l), Self::Float(r)) => {
        self.perform_float_op(operator, *l, *r)
      }
      (Self::Integer(l), Self::Float(r)) => {
        self.perform_float_op(operator, *l as f64, *r)
      }
      (Self::Float(l), Self::Integer(r)) => {
        self.perform_float_op(operator, *l, *r as f64)
      }
      (Self::String(l), r) | (r, Self::String(l))
        if operator == BinaryOperator::Add =>
      {
        let r_str = r.to_string();
        let mut res = String::with_capacity(l.len() + r_str.len());
        res.push_str(l);
        res.push_str(&r_str);
        Ok(Self::String(res.into()))
      }
      (Self::String(l), Self::String(r)) => {
        Self::compare_numbers(l, r, operator)
          .map(Self::Boolean)
          .ok_or_else(|| self.unsupported_bin_op(other, operator))
      }
      (Self::Boolean(l), Self::Boolean(r)) => match operator {
        BinaryOperator::Equal => Ok(Self::Boolean(l == r)),
        BinaryOperator::NotEqual => Ok(Self::Boolean(l != r)),
        _ => Err(self.unsupported_bin_op(other, operator)),
      },
      (l, r) => match operator {
        BinaryOperator::Equal => Ok(Self::Boolean(l == r)),
        BinaryOperator::NotEqual => Ok(Self::Boolean(l != r)),
        BinaryOperator::In => {
          if let (Self::String(l), Self::Object { properties, .. }) = (l, r) {
            return Ok(Self::Boolean(properties.read().contains_key(l)));
          }
          Err(ErrorKind::TypeError(format!(
            "Unsupported types for 'in' operation: {} and {}",
            l.type_name(),
            r.type_name()
          )))
        }
        _ => Err(self.unsupported_bin_op(other, operator)),
      },
    }
  }

  #[inline(always)]
  pub(super) fn compare_numbers<N: PartialOrd + PartialEq>(
    l: N,
    r: N,
    op: BinaryOperator,
  ) -> Option<bool> {
    match op {
      BinaryOperator::Equal => Some(l == r),
      BinaryOperator::NotEqual => Some(l != r),
      BinaryOperator::GreaterThan => Some(l > r),
      BinaryOperator::GreaterThanOrEqual => Some(l >= r),
      BinaryOperator::LessThan => Some(l < r),
      BinaryOperator::LessThanOrEqual => Some(l <= r),
      _ => None,
    }
  }

  #[inline(always)]
  pub(super) fn unsupported_bin_op(
    &self,
    other: &Self,
    op: BinaryOperator,
  ) -> ErrorKind {
    ErrorKind::TypeError(format!(
      "Unsupported types for binary operation: {} and {} with {:?}",
      self.type_name(),
      other.type_name(),
      op
    ))
  }

  pub(super) fn perform_int_op(
    &self,
    op: BinaryOperator,
    l: i64,
    r: i64,
  ) -> ErrorKindResult<Self> {
    if let Some(res) = Self::compare_numbers(l, r, op) {
      return Ok(Self::Boolean(res));
    }

    match op {
      BinaryOperator::Add => Ok(Self::Integer(l.wrapping_add(r))),
      BinaryOperator::Subtract => Ok(Self::Integer(l.wrapping_sub(r))),
      BinaryOperator::Multiply => Ok(Self::Integer(l.wrapping_mul(r))),
      BinaryOperator::Divide => {
        if r == 0 {
          return Err(ErrorKind::RuntimeError("Division by zero".into()));
        }
        Ok(Self::Integer(l.wrapping_div(r)))
      }
      BinaryOperator::Modulo => Ok(Self::Integer(l.wrapping_rem(r))),
      BinaryOperator::Exponent => {
        if r < 0 {
          return Err(ErrorKind::RuntimeError(
            "Negative exponent for integer".into(),
          ));
        }
        Ok(Self::Integer(l.pow(r as u32)))
      }
      BinaryOperator::BitwiseAnd => Ok(Self::Integer(l.bitand(r))),
      BinaryOperator::BitwiseOr => Ok(Self::Integer(l.bitor(r))),
      BinaryOperator::BitwiseXor => Ok(Self::Integer(l.bitxor(r))),
      BinaryOperator::LeftShift => Ok(Self::Integer(l.shl(r as u32))),
      BinaryOperator::RightShift => Ok(Self::Integer(l.shr(r as u32))),
      BinaryOperator::Range => {
        let elements: Vec<_> = (l..=r).map(Self::Integer).collect();
        Ok(Self::Tuple(elements.into()))
      }
      _ => Err(self.unsupported_bin_op(&Self::Integer(r), op)),
    }
  }

  pub(super) fn perform_float_op(
    &self,
    op: BinaryOperator,
    l: f64,
    r: f64,
  ) -> ErrorKindResult<Self> {
    use std::ops::{Add, Div, Mul, Rem, Sub};
    if let Some(res) = Self::compare_numbers(l, r, op) {
      return Ok(Self::Boolean(res));
    }

    match op {
      BinaryOperator::Add => Ok(Self::Float(l.add(r))),
      BinaryOperator::Subtract => Ok(Self::Float(l.sub(r))),
      BinaryOperator::Multiply => Ok(Self::Float(l.mul(r))),
      BinaryOperator::Divide => Ok(Self::Float(l.div(r))),
      BinaryOperator::Modulo => Ok(Self::Float(l.rem(r))),
      BinaryOperator::Exponent => Ok(Self::Float(l.powf(r))),
      _ => Err(self.unsupported_bin_op(&Self::Float(r), op)),
    }
  }
}
