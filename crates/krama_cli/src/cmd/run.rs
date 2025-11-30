use anyhow::Result;
use bumpalo::Bump;
use clap::Parser;
use krama_runtime::interpreter::Interpreter;

use super::read_file_and_interpret;

#[derive(Parser)]
pub struct Run {
  #[clap(default_value = "src/main.km")]
  pub file: String,
}

impl Run {
  pub async fn execute(&self) -> Result<()> {
    let arena = Bump::new();
    let interpreter = Interpreter::new(&arena, Some(self.file.as_str()));
    read_file_and_interpret(&arena, &self.file, &interpreter, true).await
  }
}
