use std::rc::Rc;

use bumpalo::Bump;
use krama_core::{ErrorKind, Object};
use krama_macro::register_global;

#[register_global("Ok")]
pub async fn ok_constructor<'ast>(
  _: &'ast Bump,
  objects: &'ast [Object<'ast>],
) -> Result<Object<'ast>, ErrorKind> {
  if objects.len() != 1 {
    return Err(ErrorKind::ArgumentError(
      "Expected 1 argument for Ok".to_string(),
    ));
  }
  Ok(Object::Ok(Rc::new(objects[0].clone())))
}

#[register_global("Err")]
pub async fn err_constructor<'ast>(
  _: &'ast Bump,
  objects: &'ast [Object<'ast>],
) -> Result<Object<'ast>, ErrorKind> {
  if objects.len() != 1 {
    return Err(ErrorKind::ArgumentError(
      "Expected 1 argument for Err".to_string(),
    ));
  }
  Ok(Object::Err(Rc::new(objects[0].clone())))
}
