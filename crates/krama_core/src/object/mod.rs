use std::sync::Arc;

use indexmap::IndexMap;
use parking_lot::RwLock;
use strum_macros::EnumProperty as EnumPropertyMacro;

use crate::Type;

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

/// The fundamental value type in the language.
/// Uses `ObjectKind` to represent everything from primitives to complex structures.
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
  String(String),
  #[strum(props(name = "array"))]
  Array {
    elements: Arc<RwLock<Vec<ObjectKind>>>,
    kind: Type,
    constant: bool,
  },
  #[strum(props(name = "tuple"))]
  Tuple {
    elements: Vec<ObjectKind>,
  },
  #[strum(props(name = "object"))]
  Object {
    properties: Arc<RwLock<IndexMap<String, ObjectKind>>>,
    definition: Option<Arc<Struct>>,
    constant: bool,
  },
  Scope(Arc<RwLock<Scope>>),
  #[strum(props(name = "function"))]
  Function(FunctionKind),
  #[strum(props(name = "return"))]
  Return(Box<ObjectKind>),
  #[strum(props(name = "break"))]
  Break,
  #[strum(props(name = "continue"))]
  Continue,
  #[strum(props(name = "ok"))]
  Ok(Box<ObjectKind>),
  #[strum(props(name = "err"))]
  Err(Box<ObjectKind>),
  #[strum(props(name = "enum"))]
  Enum {
    name: String,
    variant: String,
    fields: Option<Vec<ObjectKind>>,
  },
  #[strum(props(name = "struct"))]
  Struct(Arc<Struct>),
  #[strum(props(name = "type"))]
  Type(Type),
}
