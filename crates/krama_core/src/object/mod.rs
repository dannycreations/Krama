use std::sync::Arc;

use indexmap::IndexMap;
use parking_lot::RwLock;
use strum_macros::EnumProperty as EnumPropertyMacro;

use crate::{ErrorResult, Type};

mod behaviour;
mod collection;
mod function;
mod scope;
mod standard;
mod utils;

pub use collection::*;
pub use function::*;
pub use scope::*;
pub use standard::*;

pub type ObjectResult = ErrorResult<ObjectKind>;

/// Represents a structure definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Struct {
  pub name: Arc<str>,
  pub fields: Vec<crate::StructField>,
  pub methods: Vec<crate::StructMethod>,
  pub field_map: IndexMap<Arc<str>, usize>,
}

/// Represents an enum definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Enum {
  pub name: Arc<str>,
  pub variant: Arc<str>,
  pub field_count: usize,
}

/// Represents an instance of an enum variant.
/// Boxed in ObjectKind to reduce the size of the ObjectKind enum.
#[derive(Debug, Clone, PartialEq)]
pub struct EnumInstance {
  pub name: Arc<str>,
  pub variant: Arc<str>,
  pub fields: Option<Arc<[ObjectKind]>>,
}

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
