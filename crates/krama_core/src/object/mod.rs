use bumpalo::collections::Vec as BumpVec;
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
pub enum ObjectKind<'ast> {
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
  String(&'ast str),
  #[strum(props(name = "array"))]
  Array {
    elements: &'ast RwLock<BumpVec<'ast, ObjectKind<'ast>>>,
    kind: Type<'ast>,
    constant: bool,
  },
  #[strum(props(name = "tuple"))]
  Tuple {
    elements: &'ast [ObjectKind<'ast>],
  },
  #[strum(props(name = "object"))]
  Object {
    properties: &'ast RwLock<IndexMap<&'ast str, ObjectKind<'ast>>>,
    constant: bool,
  },
  Scope(&'ast Scope<'ast>),
  #[strum(props(name = "function"))]
  Function(FunctionKind<'ast>),
  #[strum(props(name = "return"))]
  Return(&'ast ObjectKind<'ast>),
  #[strum(props(name = "break"))]
  Break,
  #[strum(props(name = "continue"))]
  Continue,
  #[strum(props(name = "ok"))]
  Ok(&'ast ObjectKind<'ast>),
  #[strum(props(name = "err"))]
  Err(&'ast ObjectKind<'ast>),
  #[strum(props(name = "enum"))]
  Enum {
    name: &'ast str,
    variant: &'ast str,
    fields: Option<&'ast [ObjectKind<'ast>]>,
  },
  #[strum(props(name = "struct"))]
  Struct(&'ast Struct<'ast>),
  #[strum(props(name = "struct_instance"))]
  StructInstance {
    definition: &'ast Struct<'ast>,
    fields: &'ast RwLock<IndexMap<&'ast str, ObjectKind<'ast>>>,
  },
  #[strum(props(name = "type"))]
  Type(Type<'ast>),
}
