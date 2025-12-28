use std::time::Duration;

use krama_core::Object;
use krama_macro::register_module;
use tokio::time::sleep as tokio_sleep;

#[register_module(name = "sleep", module = "time")]
pub async fn sleep(objects: &[Object]) -> ObjectResult {
  parse_args!(objects, "sleep"; ms: Object::Integer(ms));
  tokio_sleep(Duration::from_millis(*ms as u64)).await;
  Ok(Object::Void)
}
