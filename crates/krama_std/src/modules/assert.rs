use bumpalo::Bump;
use futures::future::{FutureExt, LocalBoxFuture};
use krama_core::{
  error::{Error, ErrorKind},
  object::{NativeFunctionCb, Object},
  span::Span,
};
use rustc_hash::FxHashMap;

use crate::{build_native_functions, parse_args};

pub fn get_exports<'ast>() -> FxHashMap<&'static str, Object<'ast>> {
  let functions: &[(&'static str, NativeFunctionCb<'ast>)] =
    &[("assert", assert), ("assertEqual", assert_eq)];
  build_native_functions(functions)
}

fn create_assertion_error(message: String, span: Span) -> Error {
  Error {
    span,
    kind: ErrorKind::RuntimeError(message),
    file_path: None,
    source: None,
  }
}

fn assert<'ast>(
  _: &'ast Bump,
  span: Span,
  objects: &'ast [Object<'ast>],
) -> LocalBoxFuture<'ast, Result<Object<'ast>, Error>> {
  async move {
    parse_args!(objects, span; condition: condition);
    if !condition.is_truthy() {
      return Err(create_assertion_error(
        "Assertion failed: condition is not truthy".to_string(),
        span,
      ));
    }

    Ok(Object::Void)
  }
  .boxed_local()
}

fn assert_eq<'ast>(
  _: &'ast Bump,
  span: Span,
  objects: &'ast [Object<'ast>],
) -> LocalBoxFuture<'ast, Result<Object<'ast>, Error>> {
  async move {
    parse_args!(objects, span; a: a, b: b);
    if a != b {
      return Err(create_assertion_error(
        format!("Assertion failed: `{}` != `{}`", a, b),
        span,
      ));
    }

    Ok(Object::Void)
  }
  .boxed_local()
}
