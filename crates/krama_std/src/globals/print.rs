use bumpalo::collections::Vec as BumpVec;
use bumpalo::Bump;
use futures::future::LocalBoxFuture;
use itertools::Itertools;
use krama_core::error::Error;
use krama_core::error::ErrorKind;
use krama_core::object::Object;
use tokio::io;
use tokio::io::AsyncWriteExt;

pub fn print<'ast>(
  _: &'ast Bump,
  objects: BumpVec<'ast, Object<'ast>>,
) -> LocalBoxFuture<'ast, Result<Object<'ast>, Error>> {
  Box::pin(async move {
    let mut stdout = io::stdout();
    let output =
      format!("{}\n", objects.iter().map(|o| o.to_string()).join(" "));
    stdout
      .write_all(output.as_bytes())
      .await
      .map_err(|e| Error {
        span: Default::default(),
        kind: ErrorKind::RuntimeError(e.to_string()),
      })?;
    Ok(Object::Void)
  })
}
