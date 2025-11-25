use crate::build_native_functions;
use bumpalo::collections::Vec as BumpVec;
use bumpalo::Bump;
use futures::future::LocalBoxFuture;
use krama_core::error::Error;
use krama_core::error::ErrorKind;
use krama_core::object::NativeFnCallback;
use krama_core::object::Object;
use rustc_hash::FxHashMap;

pub fn get_exports<'ast>() -> FxHashMap<&'static str, Object<'ast>> {
  let functions: &[(&'static str, NativeFnCallback<'ast>)] =
    &[("assert", assert), ("assertEqual", assert_eq)];
  build_native_functions(functions)
}

fn assert<'ast>(
  _: &'ast Bump,
  objects: BumpVec<'ast, Object<'ast>>,
) -> LocalBoxFuture<'ast, Result<Object<'ast>, Error>> {
  Box::pin(async move {
    if objects.len() != 1 {
      return Err(Error {
        span: Default::default(),
        kind: ErrorKind::WrongNumberOfArguments {
          expected: 1,
          got: objects.len(),
        },
      });
    }

    if !objects[0].is_truthy() {
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
  objects: BumpVec<'ast, Object<'ast>>,
) -> LocalBoxFuture<'ast, Result<Object<'ast>, Error>> {
  Box::pin(async move {
    if objects.len() != 2 {
      return Err(Error {
        span: Default::default(),
        kind: ErrorKind::WrongNumberOfArguments {
          expected: 2,
          got: objects.len(),
        },
      });
    }

    if objects[0] != objects[1] {
      return Err(Error {
        span: Default::default(),
        kind: ErrorKind::RuntimeError(format!(
          "Assertion failed: {:?} != {:?}",
          objects[0], objects[1]
        )),
      });
    }

    Ok(Object::Void)
  })
}
