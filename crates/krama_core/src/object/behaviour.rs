use std::{
  fmt::{Display, Formatter, Result as FmtResult},
  ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub},
  sync::Arc,
};

use super::{FunctionKind, NativeFunction, ObjectKind};
use crate::{BinaryOperator, ErrorKind, UnaryOperator};

impl PartialEq for NativeFunction {
  fn eq(&self, other: &Self) -> bool {
    self.name == other.name && self.callback as usize == other.callback as usize
  }
}

impl PartialEq for FunctionKind {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (FunctionKind::Native(a), FunctionKind::Native(b)) => a == b,
      (
        FunctionKind::User { func: a, .. },
        FunctionKind::User { func: b, .. },
      ) => Arc::ptr_eq(a, b),
      (FunctionKind::Enum(a), FunctionKind::Enum(b)) => Arc::ptr_eq(a, b),
      _ => false,
    }
  }
}

impl PartialEq for ObjectKind {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Integer(l), Self::Integer(r)) => l == r,
      (Self::Float(l), Self::Float(r)) => l == r,
      (Self::Boolean(l), Self::Boolean(r)) => l == r,
      (Self::String(l), Self::String(r)) => l == r,
      (Self::Array { elements: l, .. }, Self::Array { elements: r, .. }) => {
        Arc::ptr_eq(l, r)
      }
      (Self::Tuple { elements: l }, Self::Tuple { elements: r }) => {
        Arc::ptr_eq(l, r)
      }
      (
        Self::Object { properties: l, .. },
        Self::Object { properties: r, .. },
      ) => Arc::ptr_eq(l, r),
      (Self::Null, Self::Null) | (Self::Void, Self::Void) => true,
      (Self::Scope(l), Self::Scope(r)) => Arc::ptr_eq(l, r),
      (Self::Function(l), Self::Function(r)) => l == r,
      (Self::Return(l), Self::Return(r)) => l == r,
      (Self::Break, Self::Break) | (Self::Continue, Self::Continue) => true,
      (Self::Ok(l), Self::Ok(r)) | (Self::Err(l), Self::Err(r)) => l == r,
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
      (Self::Struct(l), Self::Struct(r)) => Arc::ptr_eq(l, r),
      (Self::Type(l), Self::Type(r)) => l == r,
      _ => false,
    }
  }
}

impl ObjectKind {
  /// Helper for string concatenation.
  fn concat_strings(l: &str, r: &str) -> Self {
    let mut s = String::with_capacity(l.len() + r.len());
    s.push_str(l);
    s.push_str(r);
    Self::String(s)
  }

  /// Evaluates a binary operation on this object.
  pub fn binary_op(
    &self,
    operator: BinaryOperator,
    other: &Self,
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
      (Self::String(l), Self::String(r)) => {
        self.perform_string_op(operator, l, r)
      }
      (Self::String(l), r) if operator == BinaryOperator::Add => {
        Ok(Self::concat_strings(l, &r.to_string()))
      }
      (l, Self::String(r)) if operator == BinaryOperator::Add => {
        Ok(Self::concat_strings(&l.to_string(), r))
      }
      (Self::Boolean(l), Self::Boolean(r)) => {
        self.perform_bool_op(operator, *l, *r)
      }
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
        _ => Err(ErrorKind::TypeError(format!(
          "Unsupported types for binary operation: {} and {} with {:?}",
          l.type_name(),
          r.type_name(),
          operator
        ))),
      },
    }
  }

  /// Internal helper to unify comparison logic for numbers.
  #[inline(always)]
  fn compare_numbers<N: PartialOrd>(
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

  fn perform_int_op(
    &self,
    op: BinaryOperator,
    l: i64,
    r: i64,
  ) -> Result<Self, ErrorKind> {
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
      BinaryOperator::Exponent => Ok(Self::Integer(l.pow(r as u32))),
      BinaryOperator::BitwiseAnd => Ok(Self::Integer(l.bitand(r))),
      BinaryOperator::BitwiseOr => Ok(Self::Integer(l.bitor(r))),
      BinaryOperator::BitwiseXor => Ok(Self::Integer(l.bitxor(r))),
      BinaryOperator::LeftShift => Ok(Self::Integer(l.shl(r))),
      BinaryOperator::RightShift => Ok(Self::Integer(l.shr(r))),
      BinaryOperator::Range => {
        if r < l {
          return Ok(Self::Tuple {
            elements: Arc::new(Vec::new()),
          });
        }
        let count = (r.wrapping_sub(l)) as usize + 1;
        let mut elements = Vec::with_capacity(count);
        for i in l..=r {
          elements.push(Self::Integer(i));
        }
        Ok(Self::Tuple {
          elements: Arc::new(elements),
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
  ) -> Result<Self, ErrorKind> {
    if let Some(res) = Self::compare_numbers(l, r, op) {
      return Ok(Self::Boolean(res));
    }

    match op {
      BinaryOperator::Add => Ok(Self::concat_strings(l, r)),
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

impl From<&ObjectKind> for bool {
  #[inline]
  fn from(obj: &ObjectKind) -> bool {
    obj.is_truthy()
  }
}

impl Display for ObjectKind {
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
      Self::Object {
        properties,
        definition,
        ..
      } => {
        if let Some(def) = definition {
          write!(f, "{} {{", def.name)?;
        } else {
          write!(f, "{{")?;
        }
        let properties = properties.read();
        for (i, (key, value)) in properties.iter().enumerate() {
          if i > 0 {
            write!(f, ", ")?;
          }
          write!(f, "{}: {}", key, value)?;
        }
        write!(f, "}}")
      }
      Self::Scope(s) => write!(
        f,
        "Scope({})",
        s.read().name.as_deref().unwrap_or("anonymous")
      ),
      Self::Function(kind) => match kind {
        FunctionKind::Native(n) => write!(f, "fn {}() [native]", n.name),
        FunctionKind::User { .. } => write!(f, "fn() [user]"),
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
      Self::Type(t) => write!(f, "type {}", t),
    }
  }
}
