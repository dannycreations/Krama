use crate::cmd::CommandExecutor;
use crate::error::report_error;
use anyhow::Result;
use async_trait::async_trait;
use bumpalo::Bump;
use clap::Parser;
use krama_runtime::interpreter::Interpreter;
use tokio::fs;

#[derive(Parser)]
pub struct Run {
  #[clap(default_value = "src/main.km")]
  file: String,
}

#[async_trait(?Send)]
impl CommandExecutor for Run {
  async fn execute<'a>(&self, arena: &'a Bump) -> Result<()> {
    let interpreter = Interpreter::new(arena, Some(&self.file));
    let content = fs::read_to_string(&self.file).await?;
    let content_in_arena = arena.alloc_str(&content);
    if let Err(err) = interpreter.eval(content_in_arena).await {
      report_error(&self.file, &content, err);
    }
    Ok(())
  }
}
