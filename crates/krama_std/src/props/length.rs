use futures::future::{FutureExt, LocalBoxFuture};
use krama_core::{error::ErrorKind, object::Object};

pub(super) fn length<'ast>(
  object: Object<'ast>,
) -> LocalBoxFuture<'ast, Result<Object<'ast>, ErrorKind>> {
  async move {
    match object {
      Object::Array { elements, .. } => {
        Ok(Object::Integer(elements.len() as i64))
      }
      Object::Tuple { elements } => Ok(Object::Integer(elements.len() as i64)),
      Object::String(s) => Ok(Object::Integer(s.len() as i64)),
      _ => Err(ErrorKind::TypeError(format!(
        "Cannot get length of type `{}`",
        object.type_name()
      ))),
    }
  }
  .boxed_local()
}
