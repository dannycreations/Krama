use futures::future::FutureExt;
use futures::future::LocalBoxFuture;
use krama_core::error::Error;
use krama_core::error::ErrorKind;
use krama_core::object::Object;
use rustc_hash::FxHashMap;

use super::PropFn;

pub fn get_props() -> FxHashMap<(&'static str, &'static str), PropFn> {
  let mut props = FxHashMap::default();
  props.insert(("array", "length"), length as PropFn);
  props
}

fn length<'ast>(
  object: Object<'ast>,
) -> LocalBoxFuture<'ast, Result<Object<'ast>, Error>> {
  async move {
    match object {
      Object::Array { elements, .. } => {
        Ok(Object::Integer(elements.len() as i64))
      }
      _ => Err(Error {
        span: Default::default(),
        kind: ErrorKind::TypeMismatch(
          "length property can only be used on arrays".to_string(),
        ),
      }),
    }
  }
  .boxed_local()
}
