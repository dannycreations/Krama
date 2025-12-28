use std::sync::Arc;

use futures::future::LocalBoxFuture;
use parking_lot::RwLock;

use super::kind::Object;
use crate::{
  object::{scope::Scope, types::Enum},
  ErrorKindResult, FunctionBody, Parameter, Type,
};

/// Callback type for native functions implemented in Rust.
pub type NativeFnCb =
  fn(&[Object]) -> LocalBoxFuture<'static, ErrorKindResult<Object>>;

/// Callback type for properties that return a value based on the object instance.
pub type PropertyFnCb =
  fn(Object) -> LocalBoxFuture<'static, ErrorKindResult<Object>>;

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
  pub ty: Option<Type>,
}

/// Categorizes the different types of callable objects.
#[derive(Debug, Clone)]
pub enum Function {
  Native(NativeFunction),
  User {
    func: Arc<UserFunction>,
    env: Option<Arc<RwLock<Scope>>>,
    this: Option<Arc<Object>>,
  },
  Enum(Arc<Enum>),
}

impl PartialEq for Function {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Function::Native(a), Function::Native(b)) => a == b,
      (Function::User { func: a, .. }, Function::User { func: b, .. }) => {
        Arc::ptr_eq(a, b)
      }
      (Function::Enum(a), Function::Enum(b)) => Arc::ptr_eq(a, b),
      _ => false,
    }
  }
}
