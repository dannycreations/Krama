pub mod args;
pub mod check;
pub mod repl;
pub mod run;
pub mod test;

use anyhow::{Context, Result};
use bumpalo::Bump;
use krama_core::error::report_error;
use krama_runtime::interpreter::Interpreter;
use tokio::fs;

pub async fn read_file_and_interpret<'a>(
  arena: &'a Bump,
  file: &str,
  interpreter: &Interpreter<'a>,
  eval: bool,
) -> Result<()> {
  let content = fs::read_to_string(file)
    .await
    .with_context(|| format!("Failed to read file: {}", file))?;
  let content_in_arena = arena.alloc_str(&content);

  let result = if eval {
    interpreter.eval(content_in_arena).await.map(|_| ())
  } else {
    interpreter.check(content_in_arena).map(|_| ())
  };

  if let Err(error) = result {
    report_error(file, &content, error);
  }
  Ok(())
}
