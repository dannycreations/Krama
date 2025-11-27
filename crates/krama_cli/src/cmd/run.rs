use anyhow::Result;
use bumpalo::Bump;
use clap::Parser;
use krama_runtime::interpreter::Interpreter;
use tokio::fs;

use crate::error::report_error;

#[derive(Parser)]
pub struct Run {
  #[clap(default_value = "src/main.km")]
  pub file: String,
}

impl Run {
  pub async fn execute(&self, arena: &Bump) -> Result<()> {
    let interpreter = Interpreter::new(arena, Some(&self.file));
    let content = fs::read_to_string(&self.file).await?;
    let content_in_arena = arena.alloc_str(&content);
    if let Err(err) = interpreter.eval(content_in_arena).await {
      report_error(&self.file, &content, err);
    }
    Ok(())
  }
}
