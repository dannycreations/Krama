use strum::EnumProperty;

use super::ObjectKind;

impl ObjectKind {
  /// Checks if the object is a control flow signal (Return, Break, or Continue).
  #[inline(always)]
  pub fn is_control_signal(&self) -> bool {
    matches!(self, Self::Return(_) | Self::Break | Self::Continue)
  }

  /// Checks if the object is an Err variant.
  #[inline(always)]
  pub fn is_result_err(&self) -> bool {
    matches!(self, Self::Err(_))
  }

  /// Unwraps a Return signal to its inner value. If not a Return signal, returns self.
  #[inline(always)]
  pub fn unwrap_return(&self) -> &Self {
    if let Self::Return(inner) = self {
      inner
    } else {
      self
    }
  }

  /// Specialized unwrap that converts Return(Err(e)) into Err(e).
  /// This is used for implicit error propagation where errors are automatically returned
  /// unless handled by a Try expression.
  #[inline(always)]
  pub fn unwrap_return_err(&self) -> &Self {
    if let Self::Return(inner) = self {
      if inner.is_result_err() {
        return inner;
      }
    }
    self
  }

  /// Returns the type name of the object for diagnostics and type checking.
  #[inline(always)]
  pub fn type_name(&self) -> &str {
    match self {
      Self::Enum { name, .. } => name,
      Self::Struct(def) => &def.name,
      Self::Object {
        definition: Some(def),
        ..
      } => &def.name,
      Self::Scope(s) if s.read().name.is_some() => "module",
      Self::Scope(_) => "global",
      // Fallback to strum-generated property for primitive types.
      _ => self.get_str("name").unwrap_or("unknown"),
    }
  }

  /// Determines the truthiness of an object for logical operations and control flow.
  #[inline]
  pub fn is_truthy(&self) -> bool {
    match self {
      Self::Boolean(b) => *b,
      Self::Integer(i) => *i != 0,
      Self::Float(f) => *f != 0.0,
      Self::String(s) => !s.is_empty(),
      Self::Array { elements, .. } => !elements.read().is_empty(),
      Self::Tuple { elements } => !elements.is_empty(),
      // Null, Void, and Err are always falsy.
      Self::Null | Self::Void | Self::Err(_) => false,
      _ => true,
    }
  }

  /// Sets the constant flag for container types (Array, Object).
  #[inline]
  pub fn set_constant(&mut self, constant: bool) {
    if let Self::Array { constant: c, .. } | Self::Object { constant: c, .. } =
      self
    {
      *c = constant;
    }
  }
}
