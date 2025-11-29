use bumpalo::Bump;
use futures::future::{FutureExt, LocalBoxFuture};
use krama_core::{
  error::ErrorKind,
  object::{NativeFunctionCb, Object},
};
use rustc_hash::FxHashMap;

use crate::{build_native_functions, parse_args};

pub fn get_exports<'ast>() -> FxHashMap<&'static str, Object<'ast>> {
  let functions: &[(&'static str, NativeFunctionCb<'ast>)] =
    &[("assert", assert), ("assertEqual", assert_eq)];
  build_native_functions(functions)
}

fn assert<'ast>(
  _: &'ast Bump,
  objects: &'ast [Object<'ast>],
) -> LocalBoxFuture<'ast, Result<Object<'ast>, ErrorKind>> {
  async move {
    parse_args!(objects, "assert"; condition: condition);
    if !bool::from(condition) {
      return Err(ErrorKind::RuntimeError(
        "Assertion failed: condition is not truthy".to_string(),
      ));
    }

    Ok(Object::Void)
  }
  .boxed_local()
}

fn assert_eq<'ast>(
  _: &'ast Bump,
  objects: &'ast [Object<'ast>],
) -> LocalBoxFuture<'ast, Result<Object<'ast>, ErrorKind>> {
  async move {
    parse_args!(objects, "assertEqual"; a: a, b: b);
    if a != b {
      return Err(ErrorKind::RuntimeError(format!(
        "Assertion failed: `{}` != `{}`",
        a, b
      )));
    }

    Ok(Object::Void)
  }
  .boxed_local()
}
