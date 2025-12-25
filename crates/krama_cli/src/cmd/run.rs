use anyhow::{Context, Result};
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
    let interpreter = Interpreter::new(Some(self.file.clone()));
    let content = fs::read_to_string(&self.file)
      .await
      .with_context(|| format!("Failed to read file: {}", &self.file))?;

    let result = if eval {
      interpreter.eval(&content).await.map(|_| ())
    } else {
      interpreter.check(&content).map(|_| ())
    };

    if let Err(error) = result {
      error.report();
    }
    Ok(())
  }
}
