use bumpalo::collections::Vec as BumpVec;
use bumpalo::Bump;
use futures::future::LocalBoxFuture;
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
    let mut output = String::new();
    for (i, obj) in objects.iter().enumerate() {
      if i > 0 {
        output.push(' ');
      }
      output.push_str(&obj.to_string());
    }
    output.push('\n');

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
