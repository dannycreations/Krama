use bumpalo::Bump;
use krama_core::Object;
use krama_macro::register_global;

#[register_global("Ok")]
pub async fn ok<'ast>(
  arena: &'ast Bump,
  args: &'ast [Object<'ast>],
) -> Result<Object<'ast>, ErrorKind> {
  let value = args.first().cloned().unwrap_or(Object::Void);
  Ok(Object::Ok(arena.alloc(value)))
}

#[register_global("Err")]
pub async fn err<'ast>(
  arena: &'ast Bump,
  args: &'ast [Object<'ast>],
) -> Result<Object<'ast>, ErrorKind> {
  let value = args.first().cloned().unwrap_or(Object::Void);
  Ok(Object::Err(arena.alloc(value)))
}
