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

pub async fn execute_file_command(file: &str, eval: bool) -> Result<()> {
  let arena = Bump::new();
  let interpreter = Interpreter::new(&arena, Some(file));
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
