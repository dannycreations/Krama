use anyhow::{Context, Result};
use bumpalo::Bump;
use clap::Parser;
use krama_runtime::Interpreter;
use tokio::fs;

/// Command to run or check a source file.
#[derive(Parser)]
pub struct Run {
  /// Path to the source file to execute.
  #[clap(default_value = "src/main.km")]
  pub file: String,
}

impl Run {
  /// Executes the run command.
  /// If `eval` is true, it evaluates the file; otherwise, it only performs semantic analysis.
  pub async fn execute(&self, eval: bool) -> Result<()> {
    let arena = Bump::new();
    let interpreter = Interpreter::new(&arena, Some(&self.file));
    let content = fs::read_to_string(&self.file)
      .await
      .with_context(|| format!("Failed to read file: {}", &self.file))?;

    // Allocate content into arena to ensure it lives as long as the interpreter's 'ast lifetime.
    let content_in_arena = arena.alloc_str(&content);

    let result = if eval {
      interpreter.eval(content_in_arena).await.map(|_| ())
    } else {
      interpreter.check(content_in_arena).map(|_| ())
    };

    if let Err(error) = result {
      error.report();
    }
    Ok(())
  }
}
