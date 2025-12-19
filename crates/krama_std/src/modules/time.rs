use std::time::Duration;

use bumpalo::Bump;
use krama_core::Object;
use krama_macro::register_module;

#[register_module(name = "sleep", module = "time")]
pub async fn sleep<'ast>(
  _: &'ast Bump,
  objects: &'ast [Object<'ast>],
) -> Result<Object<'ast>, ErrorKind> {
  parse_args!(objects, "sleep"; ms: Object::Integer(ms));

  // Implicitly async via tokio
  tokio::time::sleep(Duration::from_millis(*ms as u64)).await;

  Ok(Object::Void)
}
