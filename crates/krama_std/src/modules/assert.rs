use bumpalo::Bump;
use futures::future::LocalBoxFuture;
use krama_core::{
  error::{Error, ErrorKind},
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
) -> LocalBoxFuture<'ast, Result<Object<'ast>, Error>> {
  Box::pin(async move {
    parse_args!(objects, condition: condition);
    if !condition.is_truthy() {
      return Err(Error {
        span: Default::default(),
        kind: ErrorKind::RuntimeError("Assertion failed".to_string()),
      });
    }

    Ok(Object::Void)
  })
}

fn assert_eq<'ast>(
  _: &'ast Bump,
  objects: &'ast [Object<'ast>],
) -> LocalBoxFuture<'ast, Result<Object<'ast>, Error>> {
  Box::pin(async move {
    parse_args!(objects, a: a, b: b);
    if a != b {
      return Err(Error {
        span: Default::default(),
        kind: ErrorKind::RuntimeError(format!(
          "Assertion failed: {:?} != {:?}",
          a, b
        )),
      });
    }

    Ok(Object::Void)
  })
}
