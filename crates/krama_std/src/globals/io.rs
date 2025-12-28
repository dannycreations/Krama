use krama_core::{ErrorKind, ErrorKindResult, Object};
use krama_macro::register_global;
use tokio::io::{stderr, stdout, AsyncWrite, AsyncWriteExt};

macro_rules! io_try {
  ($expr:expr) => {
    $expr
      .await
      .map_err(|e| ErrorKind::RuntimeError(e.to_string()))?
  };
}

async fn write_objects<W>(
  mut writer: W,
  objects: &[Object],
) -> ErrorKindResult<()>
where
  W: AsyncWrite + Unpin,
{
  for (i, obj) in objects.iter().enumerate() {
    if i > 0 {
      io_try!(writer.write_all(b" "));
    }
    io_try!(writer.write_all(obj.to_string().as_bytes()));
  }
  io_try!(writer.write_all(b"\n"));
  io_try!(writer.flush());
  Ok(())
}

#[register_global("print")]
pub async fn print(objects: &[Object]) -> ObjectResult {
  write_objects(stdout(), &objects).await?;
  Ok(Object::Void)
}

#[register_global("eprint")]
pub async fn eprint(objects: &[Object]) -> ObjectResult {
  write_objects(stderr(), &objects).await?;
  Ok(Object::Void)
}
