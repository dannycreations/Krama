use std::sync::Arc;

use futures::future::LocalBoxFuture;
use parking_lot::RwLock;

use super::kind::ObjectKind;
use crate::{
  object::{scope::Scope, types::Enum},
  ErrorKindResult, FunctionBody, Parameter, Type,
};

/// Callback type for native functions implemented in Rust.
pub type NativeFnCb =
  fn(&[ObjectKind]) -> LocalBoxFuture<'static, ErrorKindResult<ObjectKind>>;

/// Callback type for properties that return a value based on the object instance.
pub type PropertyFnCb =
  fn(ObjectKind) -> LocalBoxFuture<'static, ErrorKindResult<ObjectKind>>;

/// Represents a function implemented in native Rust code.
#[derive(Debug, Clone, Copy)]
pub struct NativeFunction {
  pub name: &'static str,
  pub callback: NativeFnCb,
}

impl PartialEq for NativeFunction {
  fn eq(&self, other: &Self) -> bool {
    self.name == other.name && self.callback as usize == other.callback as usize
  }
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
    this: Option<Arc<ObjectKind>>,
  },
  Enum(Arc<Enum>),
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
