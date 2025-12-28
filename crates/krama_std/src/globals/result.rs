use std::sync::Arc;

use krama_core::Object;
use krama_macro::register_global;

#[register_global("Ok")]
pub async fn ok(args: &[Object]) -> ObjectResult {
  let value = args.first().cloned().unwrap_or(Object::Void);
  Ok(Object::Ok(Arc::new(value)))
}

#[register_global("Err")]
pub async fn err(args: &[Object]) -> ObjectResult {
  let value = args.first().cloned().unwrap_or(Object::Void);
  Ok(Object::Err(Arc::new(value)))
}
