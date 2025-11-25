pub mod repl;
pub mod run;
pub mod test;

use anyhow::Result;
use async_trait::async_trait;
use bumpalo::Bump;

#[async_trait(?Send)]
pub trait CommandExecutor {
  async fn execute<'a>(&self, arena: &'a Bump) -> Result<()>;
}
