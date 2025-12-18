use anyhow::{Context, Result};
use bumpalo::Bump;
use clap::Parser;
use krama_core::report_error;
use krama_runtime::Interpreter;
use tokio::fs;

#[derive(Parser)]
pub struct Run {
  #[clap(default_value = "src/main.km")]
  pub file: String,
}

impl Run {
  pub async fn execute(&self, eval: bool) -> Result<()> {
    let arena = Bump::new();
    let interpreter = Interpreter::new(&arena, Some(&self.file));
    let content = fs::read_to_string(&self.file)
      .await
      .with_context(|| format!("Failed to read file: {}", &self.file))?;
    let content_in_arena = arena.alloc_str(&content);

    let result = if eval {
      interpreter.eval(content_in_arena).await.map(|_| ())
    } else {
      interpreter.check(content_in_arena).map(|_| ())
    };

    if let Err(error) = result {
      report_error(error);
    }
    Ok(())
  }
}
