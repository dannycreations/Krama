use std::sync::Arc;

use futures::future::LocalBoxFuture;
use parking_lot::RwLock;

use crate::{
  Enum, ErrorKind, FunctionBody, ObjectKind, Parameter, Scope, Type,
};

/// Callback type for native functions implemented in Rust.
pub type NativeFnCb =
  fn(&[ObjectKind]) -> LocalBoxFuture<'static, Result<ObjectKind, ErrorKind>>;

/// Callback type for properties that return a value based on the object instance.
pub type PropertyFnCb =
  fn(ObjectKind) -> LocalBoxFuture<'static, Result<ObjectKind, ErrorKind>>;

/// Represents a function implemented in native Rust code.
#[derive(Debug, Clone, Copy)]
pub struct NativeFunction {
  pub name: &'static str,
  pub callback: NativeFnCb,
}

/// Represents a function defined by the user in source code.
#[derive(Debug, Clone, PartialEq)]
pub struct UserFunction {
  pub parameters: Vec<Parameter>,
  pub body: FunctionBody,
  pub kind: Option<Type>,
}

/// Categorizes the different types of callable objects.
#[derive(Debug, Clone)]
pub enum FunctionKind {
  Native(NativeFunction),
  User {
    func: Arc<UserFunction>,
    env: Option<Arc<RwLock<Scope>>>,
  },
  Enum(Arc<Enum>),
}
