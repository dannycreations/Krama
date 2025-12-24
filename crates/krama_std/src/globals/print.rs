use bumpalo::Bump;
use krama_core::{ErrorKind, ObjectKind};
use krama_macro::register_global;
use tokio::io::{self, AsyncWrite, AsyncWriteExt};

macro_rules! io_try {
  ($expr:expr) => {
    $expr
      .await
      .map_err(|e| krama_core::ErrorKind::RuntimeError(e.to_string()))?
  };
}

async fn write_objects<'ast, W>(
  mut writer: W,
  objects: &'ast [ObjectKind<'ast>],
) -> Result<(), ErrorKind>
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
