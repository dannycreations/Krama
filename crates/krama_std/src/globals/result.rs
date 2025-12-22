use bumpalo::Bump;
use krama_core::ObjectKind;
use krama_macro::register_global;

#[register_global("Ok")]
pub async fn ok<'ast>(
  arena: &'ast Bump,
  args: &'ast [ObjectKind<'ast>],
) -> Result<ObjectKind<'ast>, ErrorKind> {
  let value = args.first().cloned().unwrap_or(ObjectKind::Void);
  Ok(ObjectKind::Ok(arena.alloc(value)))
}

#[register_global("Err")]
pub async fn err<'ast>(
  arena: &'ast Bump,
  args: &'ast [ObjectKind<'ast>],
) -> Result<ObjectKind<'ast>, ErrorKind> {
  let value = args.first().cloned().unwrap_or(ObjectKind::Void);
  Ok(ObjectKind::Err(arena.alloc(value)))
}
