use std::sync::Arc;

use krama_core::ObjectKind;
use krama_macro::register_global;

#[register_global("Ok")]
pub async fn ok(args: &[ObjectKind]) -> ObjectResult {
  let value = args.first().cloned().unwrap_or(ObjectKind::Void);
  Ok(ObjectKind::Ok(Arc::new(value)))
}

#[register_global("Err")]
pub async fn err(args: &[ObjectKind]) -> ObjectResult {
  let value = args.first().cloned().unwrap_or(ObjectKind::Void);
  Ok(ObjectKind::Err(Arc::new(value)))
}
