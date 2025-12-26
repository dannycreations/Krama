use std::sync::Arc;

use indexmap::IndexMap;
use parking_lot::RwLock;
use strum::EnumProperty;
use strum_macros::EnumProperty as EnumPropertyMacro;

use crate::{EnumInstance, FunctionKind, Scope, Struct, Type};

/// The fundamental value type in the language.
/// Uses `ObjectKind` to represent everything from primitives to complex structures.
/// Optimized with Arc for shared ownership and Box for large variants to keep enum size small.
#[derive(Debug, Clone, EnumPropertyMacro)]
#[repr(C, u8)]
pub enum ObjectKind {
  #[strum(props(name = "null"))]
  Null,
  #[strum(props(name = "void"))]
  Void,
  #[strum(props(name = "boolean"))]
  Boolean(bool),
  #[strum(props(name = "integer"))]
  Integer(i64),
  #[strum(props(name = "float"))]
  Float(f64),
  #[strum(props(name = "string"))]
  String(Arc<str>),
  #[strum(props(name = "array"))]
  Array {
    // Use Arc<RwLock<Vec>> for interior mutability and shared ownership.
    elements: Arc<RwLock<Vec<ObjectKind>>>,
    kind: Type,
    constant: bool,
  },
  #[strum(props(name = "tuple"))]
  Tuple(Arc<[ObjectKind]>),
  #[strum(props(name = "object"))]
  Object {
    // Use Arc<RwLock<IndexMap>> to avoid cloning large maps.
    properties: Arc<RwLock<IndexMap<Arc<str>, ObjectKind>>>,
    definition: Option<Arc<Struct>>,
    constant: bool,
  },
  Scope(Arc<RwLock<Scope>>),
  #[strum(props(name = "function"))]
  Function(FunctionKind),
  #[strum(props(name = "return"))]
  Return(Arc<ObjectKind>),
  #[strum(props(name = "break"))]
  Break,
  #[strum(props(name = "continue"))]
  Continue,
  #[strum(props(name = "ok"))]
  Ok(Arc<ObjectKind>),
  #[strum(props(name = "err"))]
  Err(Arc<ObjectKind>),
  #[strum(props(name = "enum"))]
  Enum(Box<EnumInstance>),
  #[strum(props(name = "struct"))]
  Struct(Arc<Struct>),
  #[strum(props(name = "type"))]
  Type(Type),
}

impl ObjectKind {
  /// Optimized check for control signals to avoid deep matching in hot paths.
  #[inline(always)]
  pub fn is_control_signal(&self) -> bool {
    matches!(self, Self::Return(_) | Self::Break | Self::Continue)
  }

  /// Efficiently unwraps a return signal if present.
  #[inline(always)]
  pub fn unwrap_return(&self) -> &Self {
    if let Self::Return(v) = self {
      v.as_ref()
    } else {
      self
    }
  }

  /// Efficiently unwraps a return error signal if present.
  #[inline(always)]
  pub fn unwrap_return_err(&self) -> &Self {
    if let Self::Return(v) = self {
      if v.is_result_err() {
        return v.as_ref();
      }
    }
    self
  }

  /// Quick check for truthiness.
  #[inline(always)]
  pub fn is_truthy(&self) -> bool {
    match self {
      Self::Boolean(b) => *b,
      Self::Null | Self::Void | Self::Err(_) => false,
      Self::Integer(i) => *i != 0,
      Self::Float(f) => *f != 0.0 && !f.is_nan(),
      Self::String(s) => !s.is_empty(),
      Self::Array { elements, .. } => !elements.read().is_empty(),
      Self::Tuple(elements) => !elements.is_empty(),
      Self::Object { properties, .. } => !properties.read().is_empty(),
      _ => true,
    }
  }

  /// Returns the type name of the object.
  pub fn type_name(&self) -> &str {
    match self {
      Self::Enum(instance) => &instance.name,
      Self::Struct(def) => &def.name,
      Self::Object {
        definition: Some(def),
        ..
      } => &def.name,
      Self::Scope(s) if s.read().name.is_some() => "module",
      Self::Scope(_) => "global",
      _ => self.get_str("name").unwrap_or("unknown"),
    }
  }

  /// Sets the constancy of an object. Only applicable to heap-allocated collections.
  pub fn set_constant(&mut self, is_constant: bool) {
    match self {
      Self::Array { constant, .. } | Self::Object { constant, .. } => {
        *constant = is_constant;
      }
      _ => {}
    }
  }

  /// Checks if the object is a Result::Err or a Return(Result::Err).
  #[inline(always)]
  pub fn is_result_err(&self) -> bool {
    matches!(self, Self::Err(_))
  }
}

impl From<EnumInstance> for ObjectKind {
  fn from(instance: EnumInstance) -> Self {
    Self::Enum(Box::new(instance))
  }
}
