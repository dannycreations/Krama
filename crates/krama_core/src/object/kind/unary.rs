use std::ops::{Neg, Not};

use super::ObjectKind;
use crate::{ErrorKind, ErrorKindResult, UnaryOperator};

impl ObjectKind {
  pub fn unary_op(&self, operator: UnaryOperator) -> ErrorKindResult<Self> {
    match operator {
      UnaryOperator::Not => Ok(Self::Boolean(!self.is_truthy())),
      UnaryOperator::Negate => match self {
        Self::Integer(i) => Ok(Self::Integer(i.wrapping_neg())),
        Self::Float(f) => Ok(Self::Float(f.neg())),
        _ => Err(ErrorKind::TypeError(
          "Unary '-' operator can only be applied to numbers".into(),
        )),
      },
      UnaryOperator::BitwiseNot => match self {
        Self::Integer(i) => Ok(Self::Integer(i.not())),
        _ => Err(ErrorKind::TypeError(
          "Bitwise not operator can only be applied to integers".into(),
        )),
      },
    }
  }
}
