use std::sync::Arc;

use indexmap::IndexMap;
use parking_lot::RwLock;
use strum::EnumProperty;
use strum_macros::EnumProperty as EnumPropertyMacro;

use super::{
  function::FunctionKind,
  scope::Scope,
  types::{EnumInstance, Struct},
};
use crate::{LiteralKind, Type};

mod binary;
mod display;
mod unary;

/// The fundamental value type in the language.
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
    elements: Arc<RwLock<Vec<ObjectKind>>>,
    kind: Type,
    constant: bool,
  },
  #[strum(props(name = "tuple"))]
  Tuple(Arc<[ObjectKind]>),
  #[strum(props(name = "object"))]
  Object {
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
  #[inline(always)]
  pub fn is_control_signal(&self) -> bool {
    matches!(self, Self::Return(_) | Self::Break | Self::Continue)
  }

  #[inline(always)]
  pub fn unwrap_return(&self) -> &Self {
    if let Self::Return(v) = self {
      v.as_ref()
    } else {
      self
    }
  }

  #[inline(always)]
  pub fn unwrap_return_err(&self) -> &Self {
    if let Self::Return(v) = self {
      if v.is_result_err() {
        return v.as_ref();
      }
    }
    self
  }

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

  pub fn set_constant(&mut self, is_constant: bool) {
    match self {
      Self::Array { constant, .. } | Self::Object { constant, .. } => {
        *constant = is_constant;
      }
      _ => {}
    }
  }

  #[inline(always)]
  pub fn is_result_err(&self) -> bool {
    matches!(self, Self::Err(_))
  }
}

impl PartialEq for ObjectKind {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Integer(l), Self::Integer(r)) => l == r,
      (Self::Float(l), Self::Float(r)) => l == r,
      (Self::Boolean(l), Self::Boolean(r)) => l == r,
      (Self::String(l), Self::String(r)) => Arc::ptr_eq(l, r) || *l == *r,
      (Self::Array { elements: l, .. }, Self::Array { elements: r, .. }) => {
        Arc::ptr_eq(l, r)
      }
      (Self::Tuple(l), Self::Tuple(r)) => Arc::ptr_eq(l, r),
      (
        Self::Object { properties: l, .. },
        Self::Object { properties: r, .. },
      ) => Arc::ptr_eq(l, r),
      (Self::Null, Self::Null) | (Self::Void, Self::Void) => true,
      (Self::Scope(l), Self::Scope(r)) => Arc::ptr_eq(l, r),
      (Self::Function(l), Self::Function(r)) => l == r,
      (Self::Return(l), Self::Return(r)) => Arc::ptr_eq(l, r) || l == r,
      (Self::Break, Self::Break) | (Self::Continue, Self::Continue) => true,
      (Self::Ok(l), Self::Ok(r)) | (Self::Err(l), Self::Err(r)) => {
        Arc::ptr_eq(l, r) || l == r
      }
      (Self::Enum(l), Self::Enum(r)) => l == r,
      (Self::Struct(l), Self::Struct(r)) => Arc::ptr_eq(l, r),
      (Self::Type(l), Self::Type(r)) => l == r,
      _ => false,
    }
  }
}

impl From<EnumInstance> for ObjectKind {
  fn from(instance: EnumInstance) -> Self {
    Self::Enum(Box::new(instance))
  }
}

impl From<LiteralKind> for ObjectKind {
  fn from(literal: LiteralKind) -> Self {
    match literal {
      LiteralKind::Integer(i) => Self::Integer(i),
      LiteralKind::Float(f) => Self::Float(f),
      LiteralKind::String(s) => Self::String(s),
      LiteralKind::Boolean(b) => Self::Boolean(b),
      LiteralKind::Null => Self::Null,
    }
  }
}

impl From<&ObjectKind> for bool {
  #[inline]
  fn from(obj: &ObjectKind) -> bool {
    obj.is_truthy()
  }
}
