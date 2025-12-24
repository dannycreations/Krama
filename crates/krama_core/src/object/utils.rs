use super::ObjectKind;

impl<'ast> ObjectKind<'ast> {
  /// Checks if the object is a control flow signal.
  #[inline(always)]
  pub fn is_control_signal(&self) -> bool {
    matches!(self, Self::Return(_) | Self::Break | Self::Continue)
  }

  /// Checks if the object is an Err variant.
  #[inline(always)]
  pub fn is_result_err(&self) -> bool {
    matches!(self, Self::Err(_))
  }

  /// Helper to unwrap Return signals if they contain an Err variant.
  #[inline(always)]
  pub fn unwrap_return_err(&self) -> &Self {
    if let Self::Return(inner) = self {
      if inner.is_result_err() {
        return inner;
      }
    }
    self
  }

  /// Helper to unwrap Return signals.
  #[inline(always)]
  pub fn unwrap_return(&self) -> &Self {
    if let Self::Return(inner) = self {
      inner
    } else {
      self
    }
  }

  /// Returns the type name of the object.
  #[inline(always)]
  pub fn type_name(&self) -> &str {
    use strum::EnumProperty;
    match self {
      Self::Enum { name, .. } => name,
      Self::Struct(def) => def.name,
      Self::StructInstance { definition, .. } => definition.name,
      Self::Scope(s) if s.name.is_some() => "module",
      Self::Scope(_) => "global",
      _ => self.get_str("name").unwrap_or("unknown"),
    }
  }

  /// Checks if the object is truthy.
  #[inline]
  pub fn is_truthy(&self) -> bool {
    match self {
      Self::Boolean(b) => *b,
      Self::Integer(i) => *i != 0,
      Self::Float(f) => *f != 0.0,
      Self::String(s) => !s.is_empty(),
      Self::Array { elements, .. } => !elements.read().is_empty(),
      Self::Tuple { elements } => !elements.is_empty(),
      Self::Null | Self::Void | Self::Err(_) => false,
      _ => true,
    }
  }

  /// Sets the constant flag for containers.
  #[inline]
  pub fn set_constant(&mut self, constant: bool) {
    if let Self::Array { constant: c, .. } | Self::Object { constant: c, .. } =
      self
    {
      *c = constant;
    }
  }
}
