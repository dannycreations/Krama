use bumpalo::Bump;
use krama_core::{ErrorKind, Object};
use krama_macro::register_global;
use tokio::{io, io::AsyncWriteExt};

#[register_global("print")]
pub async fn print<'ast>(
  _: &'ast Bump,
  objects: &'ast [Object<'ast>],
) -> Result<Object<'ast>, ErrorKind> {
  let mut stdout = io::stdout();
  for (i, obj) in objects.iter().enumerate() {
    if i > 0 {
      stdout
        .write_all(b" ")
        .await
        .map_err(|e| ErrorKind::RuntimeError(e.to_string()))?;
    }
    stdout
      .write_all(obj.to_string().as_bytes())
      .await
      .map_err(|e| ErrorKind::RuntimeError(e.to_string()))?;
  }
  stdout
    .write_all(b"\n")
    .await
    .map_err(|e| ErrorKind::RuntimeError(e.to_string()))?;

  Ok(Object::Void)
}
