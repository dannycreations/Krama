use std::{
  env,
  io::Error,
  path::{Path, PathBuf},
};

use anyhow::{anyhow, Context, Result};
use bumpalo::Bump;
use clap::Parser;
use krama_runtime::{interpreter::Interpreter, testing::TestResult};
use tokio::fs;
use walkdir::WalkDir;

use crate::error::report_error;

#[derive(Parser)]
pub struct Test {
  #[clap(default_value = "src")]
  pub path: String,
}

fn find_test_files(path: &Path) -> Result<Vec<PathBuf>, Error> {
  let mut test_files = Vec::new();
  for entry in WalkDir::new(path) {
    let entry = entry?;
    let path = entry.path();
    if path.is_file() && path.to_string_lossy().ends_with("_test.km") {
      test_files.push(path.to_path_buf());
    }
  }
  Ok(test_files)
}

impl Test {
  pub async fn execute(&self, arena: &mut Bump) -> Result<()> {
    let mut passed = 0;
    let mut failed = 0;

    let root_path = env::current_dir()?;
    let mut path_buf = root_path;
    path_buf.push(&self.path);
    let path = path_buf.as_path();

    let test_files = find_test_files(path)?;

    for path_buf in test_files {
      let path = path_buf.as_path();
      let content = fs::read_to_string(&path).await?;
      let path_str = path.to_str().context("path is not valid UTF-8")?;

      println!("Running tests in {}", path.display());
      {
        let interpreter = Interpreter::new(arena, Some(path_str));
        let program = interpreter.parse_and_resolve(&content)?;
        let results = interpreter.run_tests(&program.statements).await;
        for result in results {
          match result {
            TestResult::Success(name) => {
              println!("  test {} ... ok", name);
              passed += 1;
            }
            TestResult::Failure(name, err) => {
              println!("  '{}'... failed", name);
              report_error(path_str, &content, err);
              failed += 1;
            }
          }
        }
      }
      arena.reset();
    }
    println!("\nTest results: {} passed, {} failed", passed, failed);

    if failed > 0 {
      return Err(anyhow!("{} test(s) failed", failed));
    }

    Ok(())
  }
}
