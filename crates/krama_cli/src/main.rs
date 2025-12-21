use anyhow::Result;
use clap::Parser;
use krama_cli::Command;

#[tokio::main]
async fn main() -> Result<()> {
  let cmd = Command::parse();
  cmd.execute().await
}
