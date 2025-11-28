use bumpalo::Bump;
use futures::future::LocalBoxFuture;
use krama_core::{
  error::{Error, ErrorKind},
  object::Object,
};
use tokio::{io, io::AsyncWriteExt};

pub fn print<'ast>(
  _: &'ast Bump,
  objects: &'ast [Object<'ast>],
) -> LocalBoxFuture<'ast, Result<Object<'ast>, Error>> {
  Box::pin(async move {
    let mut stdout = io::stdout();
    for (i, obj) in objects.iter().enumerate() {
      if i > 0 {
        stdout.write_all(b" ").await.map_err(|e| Error {
          span: Default::default(),
          kind: ErrorKind::RuntimeError(e.to_string()),
        })?;
      }
      stdout
        .write_all(obj.to_string().as_bytes())
        .await
        .map_err(|e| Error {
          span: Default::default(),
          kind: ErrorKind::RuntimeError(e.to_string()),
        })?;
    }
    stdout.write_all(b"\n").await.map_err(|e| Error {
      span: Default::default(),
      kind: ErrorKind::RuntimeError(e.to_string()),
    })?;

    Ok(Object::Void)
  })
}
