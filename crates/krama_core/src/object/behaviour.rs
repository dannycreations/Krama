use std::{
  fmt::{Display, Formatter, Result as FmtResult},
  ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub},
  ptr,
};

use bumpalo::{
  collections::{String as BumpString, Vec as BumpVec},
  Bump,
};

use super::{FunctionKind, NativeFunction, ObjectKind};
use crate::{BinaryOperator, ErrorKind, UnaryOperator};

impl PartialEq for NativeFunction {
  fn eq(&self, other: &Self) -> bool {
    self.name == other.name && self.callback as usize == other.callback as usize
  }
}

impl<'ast> PartialEq for FunctionKind<'ast> {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (FunctionKind::Native(a), FunctionKind::Native(b)) => a == b,
      (FunctionKind::User(a), FunctionKind::User(b)) => ptr::eq(*a, *b),
      (FunctionKind::Enum(a), FunctionKind::Enum(b)) => ptr::eq(*a, *b),
      _ => false,
    }
  }
}

impl<'ast> PartialEq for ObjectKind<'ast> {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Integer(l), Self::Integer(r)) => l == r,
      (Self::Float(l), Self::Float(r)) => l == r,
      (Self::Boolean(l), Self::Boolean(r)) => l == r,
      (Self::String(l), Self::String(r)) => ptr::eq(*l, *r) || l == r,
      (Self::Array { elements: l, .. }, Self::Array { elements: r, .. }) => {
        ptr::eq(*l, *r)
      }
      (Self::Tuple { elements: l }, Self::Tuple { elements: r }) => {
        ptr::eq(*l, *r) || l == r
      }
      (
        Self::Object { properties: l, .. },
        Self::Object { properties: r, .. },
      ) => ptr::eq(*l, *r),
      (Self::Null, Self::Null) | (Self::Void, Self::Void) => true,
      (Self::Scope(l), Self::Scope(r)) => ptr::eq(*l, *r),
      (Self::Function(l), Self::Function(r)) => l == r,
      (Self::Return(l), Self::Return(r)) => ptr::eq(*l, *r),
      (Self::Break, Self::Break) | (Self::Continue, Self::Continue) => true,
      (Self::Ok(l), Self::Ok(r)) | (Self::Err(l), Self::Err(r)) => {
        ptr::eq(*l, *r) || l == r
      }
      (
        Self::Enum {
          name: ln,
          variant: lv,
          fields: lf,
        },
        Self::Enum {
          name: rn,
          variant: rv,
          fields: rf,
        },
      ) => ln == rn && lv == rv && lf == rf,
      (Self::Struct(l), Self::Struct(r)) => ptr::eq(*l, *r),
      (
        Self::StructInstance { fields: lf, .. },
        Self::StructInstance { fields: rf, .. },
      ) => ptr::eq(*lf, *rf),
      (Self::Type(l), Self::Type(r)) => l == r,
      _ => false,
    }
  }
}

impl<'ast> ObjectKind<'ast> {
  /// Helper for string concatenation.
  fn concat_strings(l: &str, r: &str, arena: &'ast Bump) -> Self {
    let mut s = BumpString::with_capacity_in(l.len() + r.len(), arena);
    s.push_str(l);
    s.push_str(r);
    Self::String(s.into_bump_str())
  }

  /// Evaluates a binary operation on this object.
  pub fn binary_op(
    &self,
    operator: BinaryOperator,
    other: &Self,
    arena: &'ast Bump,
  ) -> Result<Self, ErrorKind> {
    // Propagate early exits from either side.
    if self.is_control_signal() {
      return Ok(self.clone());
    }
    if other.is_control_signal() {
      return Ok(other.clone());
    }

    match (self, other) {
      (Self::Integer(l), Self::Integer(r)) => {
        self.perform_int_op(operator, *l, *r, arena)
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
      (Self::String(l), Self::String(r)) => {
        self.perform_string_op(operator, l, r, arena)
      }
      (Self::String(l), r) if operator == BinaryOperator::Add => {
        Ok(Self::concat_strings(l, &r.to_string(), arena))
      }
      (l, Self::String(r)) if operator == BinaryOperator::Add => {
        Ok(Self::concat_strings(&l.to_string(), r, arena))
      }
      (Self::Boolean(l), Self::Boolean(r)) => {
        self.perform_bool_op(operator, *l, *r)
      }
      (l, r) => match operator {
        BinaryOperator::Equal => Ok(Self::Boolean(l == r)),
        BinaryOperator::NotEqual => Ok(Self::Boolean(l != r)),
        BinaryOperator::In => {
          if let (Self::String(l), Self::Object { properties, .. }) = (l, r) {
            return Ok(Self::Boolean(properties.read().contains_key(*l)));
          }
          Err(ErrorKind::TypeError(format!(
            "Unsupported types for 'in' operation: {} and {}",
            l.type_name(),
            r.type_name()
          )))
        }
        _ => Err(ErrorKind::TypeError(format!(
          "Unsupported types for binary operation: {} and {} with {:?}",
          l.type_name(),
          r.type_name(),
          operator
        ))),
      },
    }
  }

  fn perform_int_op(
    &self,
    op: BinaryOperator,
    l: i64,
    r: i64,
    arena: &'ast Bump,
  ) -> Result<Self, ErrorKind> {
    match op {
      BinaryOperator::Add => Ok(Self::Integer(l.add(r))),
      BinaryOperator::Subtract => Ok(Self::Integer(l.sub(r))),
      BinaryOperator::Multiply => Ok(Self::Integer(l.mul(r))),
      BinaryOperator::Divide => {
        if r == 0 {
          return Err(ErrorKind::RuntimeError("Division by zero".into()));
        }
        Ok(Self::Integer(l.div(r)))
      }
      BinaryOperator::Modulo => Ok(Self::Integer(l.rem(r))),
      BinaryOperator::Exponent => Ok(Self::Integer(l.pow(r as u32))),
      BinaryOperator::BitwiseAnd => Ok(Self::Integer(l.bitand(r))),
      BinaryOperator::BitwiseOr => Ok(Self::Integer(l.bitor(r))),
      BinaryOperator::BitwiseXor => Ok(Self::Integer(l.bitxor(r))),
      BinaryOperator::LeftShift => Ok(Self::Integer(l.shl(r))),
      BinaryOperator::RightShift => Ok(Self::Integer(l.shr(r))),
      BinaryOperator::Equal => Ok(Self::Boolean(l == r)),
      BinaryOperator::NotEqual => Ok(Self::Boolean(l != r)),
      BinaryOperator::GreaterThan => Ok(Self::Boolean(l > r)),
      BinaryOperator::GreaterThanOrEqual => Ok(Self::Boolean(l >= r)),
      BinaryOperator::LessThan => Ok(Self::Boolean(l < r)),
      BinaryOperator::LessThanOrEqual => Ok(Self::Boolean(l <= r)),
      BinaryOperator::Range => {
        if r < l {
          return Ok(Self::Tuple { elements: &[] });
        }
        let count = (r - l) as usize + 1;
        let mut elements = BumpVec::with_capacity_in(count, arena);
        for i in l..=r {
          elements.push(Self::Integer(i));
        }
        Ok(Self::Tuple {
          elements: elements.into_bump_slice(),
        })
      }
      _ => Err(ErrorKind::TypeError(format!(
        "Unsupported operator for integers: {:?}",
        op
      ))),
    }
  }

  fn perform_float_op(
    &self,
    op: BinaryOperator,
    l: f64,
    r: f64,
  ) -> Result<Self, ErrorKind> {
    match op {
      BinaryOperator::Add => Ok(Self::Float(l.add(r))),
      BinaryOperator::Subtract => Ok(Self::Float(l.sub(r))),
      BinaryOperator::Multiply => Ok(Self::Float(l.mul(r))),
      BinaryOperator::Divide => Ok(Self::Float(l.div(r))),
      BinaryOperator::Modulo => Ok(Self::Float(l.rem(r))),
      BinaryOperator::Exponent => Ok(Self::Float(l.powf(r))),
      BinaryOperator::Equal => Ok(Self::Boolean(l == r)),
      BinaryOperator::NotEqual => Ok(Self::Boolean(l != r)),
      BinaryOperator::GreaterThan => Ok(Self::Boolean(l > r)),
      BinaryOperator::GreaterThanOrEqual => Ok(Self::Boolean(l >= r)),
      BinaryOperator::LessThan => Ok(Self::Boolean(l < r)),
      BinaryOperator::LessThanOrEqual => Ok(Self::Boolean(l <= r)),
      _ => Err(ErrorKind::TypeError(format!(
        "Unsupported operator for floats: {:?}",
        op
      ))),
    }
  }

  fn perform_string_op(
    &self,
    op: BinaryOperator,
    l: &str,
    r: &str,
    arena: &'ast Bump,
  ) -> Result<Self, ErrorKind> {
    match op {
      BinaryOperator::Add => Ok(Self::concat_strings(l, r, arena)),
      BinaryOperator::Equal => Ok(Self::Boolean(l == r)),
      BinaryOperator::NotEqual => Ok(Self::Boolean(l != r)),
      _ => Err(ErrorKind::TypeError(format!(
        "Unsupported operator for strings: {:?}",
        op
      ))),
    }
  }

  fn perform_bool_op(
    &self,
    op: BinaryOperator,
    l: bool,
    r: bool,
  ) -> Result<Self, ErrorKind> {
    match op {
      BinaryOperator::Equal => Ok(Self::Boolean(l == r)),
      BinaryOperator::NotEqual => Ok(Self::Boolean(l != r)),
      _ => Err(ErrorKind::TypeError(format!(
        "Unsupported operator for booleans: {:?}",
        op
      ))),
    }
  }

  /// Evaluates a unary operation on this object.
  pub fn unary_op(&self, operator: UnaryOperator) -> Result<Self, ErrorKind> {
    match operator {
      UnaryOperator::Not => Ok(Self::Boolean(!self.is_truthy())),
      UnaryOperator::Negate => match self {
        Self::Integer(i) => Ok(Self::Integer(i.neg())),
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

impl<'ast> From<&ObjectKind<'ast>> for bool {
  #[inline]
  fn from(obj: &ObjectKind<'ast>) -> bool {
    obj.is_truthy()
  }
}

impl<'ast> Display for ObjectKind<'ast> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::Null => write!(f, "null"),
      Self::Void => write!(f, "void"),
      Self::Boolean(b) => write!(f, "{}", b),
      Self::Integer(i) => write!(f, "{}", i),
      Self::Float(fl) => write!(f, "{}", fl),
      Self::String(s) => write!(f, "{}", s),
      Self::Array { elements, .. } => {
        write!(f, "[")?;
        let elements = elements.read();
        for (i, el) in elements.iter().enumerate() {
          if i > 0 {
            write!(f, ", ")?;
          }
          write!(f, "{}", el)?;
        }
        write!(f, "]")
      }
      Self::Tuple { elements } => {
        write!(f, "(")?;
        for (i, el) in elements.iter().enumerate() {
          if i > 0 {
            write!(f, ", ")?;
          }
          write!(f, "{}", el)?;
        }
        write!(f, ")")
      }
      Self::Object { properties, .. } => {
        write!(f, "{{")?;
        let properties = properties.read();
        for (i, (key, value)) in properties.iter().enumerate() {
          if i > 0 {
            write!(f, ", ")?;
          }
          write!(f, "{}: {}", key, value)?;
        }
        write!(f, "}}")
      }
      Self::Scope(s) => write!(f, "Scope({})", s.name.unwrap_or("anonymous")),
      Self::Function(kind) => match kind {
        FunctionKind::Native(n) => write!(f, "fn {}() [native]", n.name),
        FunctionKind::User(_) => write!(f, "fn() [user]"),
        FunctionKind::Enum(e) => {
          write!(f, "fn {}::{}() [enum]", e.name, e.variant)
        }
      },
      Self::Return(v) => write!(f, "return {}", v),
      Self::Break => write!(f, "break"),
      Self::Continue => write!(f, "continue"),
      Self::Ok(v) => write!(f, "Ok({})", v),
      Self::Err(v) => write!(f, "Err({})", v),
      Self::Enum {
        name,
        variant,
        fields,
      } => {
        write!(f, "{}::{}", name, variant)?;
        if let Some(fields) = fields {
          write!(f, "(")?;
          for (i, field) in fields.iter().enumerate() {
            if i > 0 {
              write!(f, ", ")?;
            }
            write!(f, "{}", field)?;
          }
          write!(f, ")")?;
        }
        Ok(())
      }
      Self::Struct(s) => write!(f, "struct {}", s.name),
      Self::StructInstance { definition, fields } => {
        write!(f, "{} {{", definition.name)?;
        let fields = fields.read();
        for (i, (name, value)) in fields.iter().enumerate() {
          if i > 0 {
            write!(f, ", ")?;
          }
          write!(f, "{}: {}", name, value)?;
        }
        write!(f, "}}")
      }
      Self::Type(t) => write!(f, "type {}", t),
    }
  }
}
