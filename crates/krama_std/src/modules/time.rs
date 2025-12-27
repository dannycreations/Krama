use std::time::Duration;

use krama_core::ObjectKind;
use krama_macro::register_module;
use tokio::time;

#[register_module(name = "sleep", module = "time")]
pub async fn sleep(objects: &[ObjectKind]) -> ObjectResult {
  parse_args!(objects, "sleep"; ms: ObjectKind::Integer(ms));
  time::sleep(Duration::from_millis(*ms as u64)).await;
  Ok(ObjectKind::Void)
}
