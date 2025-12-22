use std::fmt::Debug;

use bumpalo::{collections::Vec as BumpVec, Bump};
use futures::future::LocalBoxFuture;
use indexmap::IndexMap;
use parking_lot::RwLock;
use strum_macros::EnumProperty as EnumPropertyMacro;

use crate::{
  ErrorKind, FunctionBody, Parameter, StructField, StructMethod, Type,
};

mod behaviour;
mod scope;

pub use scope::*;

pub type NativeFnCb =
  for<'ast> fn(
    &'ast Bump,
    &'ast [ObjectKind<'ast>],
  ) -> LocalBoxFuture<'ast, Result<ObjectKind<'ast>, ErrorKind>>;

pub type PropertyFnCb =
  for<'ast> fn(
    ObjectKind<'ast>,
  ) -> LocalBoxFuture<'ast, Result<ObjectKind<'ast>, ErrorKind>>;

#[derive(Debug, Clone, Copy)]
pub struct NativeFunction {
  pub name: &'static str,
  pub callback: NativeFnCb,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserFunction<'ast> {
  pub parameters: BumpVec<'ast, Parameter<'ast>>,
  pub body: FunctionBody<'ast>,
  pub kind: Option<Type<'ast>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Enum<'ast> {
  pub name: &'ast str,
  pub variant: &'ast str,
  pub field_count: usize,
}

#[derive(Debug, Copy, Clone)]
pub enum FunctionKind<'ast> {
  Native(NativeFunction),
  User(&'ast UserFunction<'ast>),
  Enum(&'ast Enum<'ast>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Struct<'ast> {
  pub name: &'ast str,
  pub fields: BumpVec<'ast, StructField<'ast>>,
  pub methods: BumpVec<'ast, StructMethod<'ast>>,
}

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

impl<'ast> ObjectKind<'ast> {
  /// Allocates a new UserFunction from a StructMethod.
  /// Centralizes the conversion of methods to callable objects.
  pub fn from_method(
    method: &StructMethod<'ast>,
    arena: &'ast Bump,
  ) -> ObjectKind<'ast> {
    ObjectKind::Function(FunctionKind::User(arena.alloc(UserFunction {
      parameters: method.parameters.clone(),
      body: method.body.clone(),
      kind: method.kind.clone(),
    })))
  }
}

pub struct StandardGlobal {
  pub name: &'static str,
  pub callback: NativeFnCb,
}

#[linkme::distributed_slice]
pub static STANDARD_GLOBALS: [StandardGlobal];

pub struct StandardModule {
  pub name: &'static str,
  pub callback: NativeFnCb,
  pub module: &'static str,
}

#[linkme::distributed_slice]
pub static STANDARD_MODULES: [StandardModule];

pub struct StandardProperty {
  pub name: &'static str,
  pub callback: PropertyFnCb,
  pub types: &'static [&'static str],
}

#[linkme::distributed_slice]
pub static STANDARD_PROPERTIES: [StandardProperty];
