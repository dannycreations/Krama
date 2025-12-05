use bumpalo::Bump;
use krama_core::{error::ErrorKind, object::Object};
use krama_macro::register_module;

#[register_module(name = "assert", module = "assert")]
async fn assert<'ast>(
  _: &'ast Bump,
  objects: &'ast [Object<'ast>],
) -> Result<Object<'ast>, ErrorKind> {
  parse_args!(objects, "assert"; condition: condition);
  if !bool::from(condition) {
    return Err(ErrorKind::RuntimeError(
      "Assertion failed: condition is not truthy".to_string(),
    ));
  }

  Ok(Object::Void)
}

#[register_module(name = "assertEqual", module = "assert")]
async fn assert_eq<'ast>(
  _: &'ast Bump,
  objects: &'ast [Object<'ast>],
) -> Result<Object<'ast>, ErrorKind> {
  parse_args!(objects, "assertEqual"; a: a, b: b);
  if a != b {
    return Err(ErrorKind::RuntimeError(format!(
      "Assertion failed: `{}` != `{}`",
      a, b
    )));
  }

  Ok(Object::Void)
}
