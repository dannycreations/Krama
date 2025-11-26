use futures::future::{FutureExt, LocalBoxFuture};
use krama_core::{
  error::{Error, ErrorKind},
  object::Object,
};
use rustc_hash::FxHashMap;

use super::PropFn;

pub fn get_props() -> FxHashMap<(&'static str, &'static str), PropFn> {
  let mut props = FxHashMap::default();
  props.insert(("string", "length"), length as PropFn);
  props
}

fn length<'ast>(
  object: Object<'ast>,
) -> LocalBoxFuture<'ast, Result<Object<'ast>, Error>> {
  async move {
    match object {
      Object::String(s) => Ok(Object::Integer(s.len() as i64)),
      _ => Err(Error {
        span: Default::default(),
        kind: ErrorKind::TypeError(format!(
          "Cannot get length of type `{}`",
          object.type_name()
        )),
      }),
    }
  }
  .boxed_local()
}
