use bumpalo::{collections::Vec as BumpVec, Bump};
use futures::future::LocalBoxFuture;

use crate::{ErrorKind, FunctionBody, ObjectKind, Parameter, Type};

/// Callback type for native functions implemented in Rust.
pub type NativeFnCb =
  for<'ast> fn(
    &'ast Bump,
    &'ast [ObjectKind<'ast>],
  ) -> LocalBoxFuture<'ast, Result<ObjectKind<'ast>, ErrorKind>>;

/// Callback type for properties that return a value based on the object instance.
pub type PropertyFnCb =
  for<'ast> fn(
    ObjectKind<'ast>,
  ) -> LocalBoxFuture<'ast, Result<ObjectKind<'ast>, ErrorKind>>;

/// Represents a function implemented in native Rust code.
#[derive(Debug, Clone, Copy)]
pub struct NativeFunction {
  pub name: &'static str,
  pub callback: NativeFnCb,
}

/// Represents a function defined by the user in source code.
#[derive(Debug, Clone, PartialEq)]
pub struct UserFunction<'ast> {
  pub parameters: BumpVec<'ast, Parameter<'ast>>,
  pub body: FunctionBody<'ast>,
  pub kind: Option<Type<'ast>>,
}

/// Categorizes the different types of callable objects.
#[derive(Debug, Copy, Clone)]
pub enum FunctionKind<'ast> {
  Native(NativeFunction),
  User(&'ast UserFunction<'ast>),
  Enum(&'ast crate::Enum<'ast>),
}
