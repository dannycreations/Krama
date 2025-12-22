use bumpalo::Bump;
use krama_core::{ErrorKind, ObjectKind};
use krama_macro::register_global;
use tokio::io::{self, AsyncWrite, AsyncWriteExt};

/// Internal helper to handle asynchronous output to any writer.
/// Consolidates the logic for printing multiple objects separated by spaces.
async fn write_objects<'ast, W>(
  mut writer: W,
  objects: &'ast [ObjectKind<'ast>],
) -> Result<(), ErrorKind>
where
  W: AsyncWrite + Unpin,
{
  for (i, obj) in objects.iter().enumerate() {
    if i > 0 {
      writer
        .write_all(b" ")
        .await
        .map_err(|e| ErrorKind::RuntimeError(e.to_string()))?;
    }
    writer
      .write_all(obj.to_string().as_bytes())
      .await
      .map_err(|e| ErrorKind::RuntimeError(e.to_string()))?;
  }
  writer
    .write_all(b"\n")
    .await
    .map_err(|e| ErrorKind::RuntimeError(e.to_string()))?;
  writer
    .flush()
    .await
    .map_err(|e| ErrorKind::RuntimeError(e.to_string()))?;
  Ok(())
}

#[register_global("print")]
pub async fn print<'ast>(
  _: &'ast Bump,
  objects: &'ast [ObjectKind<'ast>],
) -> Result<ObjectKind<'ast>, ErrorKind> {
  write_objects(io::stdout(), objects).await?;
  Ok(ObjectKind::Void)
}

#[register_global("eprint")]
pub async fn eprint<'ast>(
  _: &'ast Bump,
  objects: &'ast [ObjectKind<'ast>],
) -> Result<ObjectKind<'ast>, ErrorKind> {
  write_objects(io::stderr(), objects).await?;
  Ok(ObjectKind::Void)
}
