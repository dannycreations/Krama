use anyhow::Result;
use bumpalo::Bump;
use clap::Parser;
use krama_runtime::interpreter::Interpreter;
use tokio::fs;

use crate::error::report_error;

#[derive(Parser)]
pub struct Check {
  #[clap(default_value = "src/main.km")]
  pub file: String,
}

impl Check {
  pub async fn execute(&self) -> Result<()> {
    let arena = Bump::new();
    let interpreter = Interpreter::new(&arena, Some(self.file.as_str()));
    let content = fs::read_to_string(&self.file).await?;
    let content_in_arena = arena.alloc_str(&content);
    if let Err((kind, span)) = interpreter.check(content_in_arena) {
      report_error(&self.file, &content, span, kind);
    }
    Ok(())
  }
}
