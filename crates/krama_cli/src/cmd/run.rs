use anyhow::Result;
use clap::Parser;

use super::execute_file_command;

#[derive(Parser)]
pub struct Run {
  #[clap(default_value = "src/main.km")]
  pub file: String,
}

impl Run {
  pub async fn execute(&self) -> Result<()> {
    execute_file_command(&self.file, true).await
  }
}
