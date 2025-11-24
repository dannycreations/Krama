use crate::error::report_error;
use anyhow::Result;
use bumpalo::Bump;
use clap::Parser;
use futures::future::BoxFuture;
use futures::FutureExt;
use krama_runtime::interpreter::Interpreter;
use std::path::Path;
use tokio::fs;

#[derive(Parser)]
pub struct Test {
  #[clap(default_value = "src")]
  path: String,
}

fn find_test_files<'a>(
  path: &'a Path,
  test_files: &'a mut Vec<std::path::PathBuf>,
) -> BoxFuture<'a, Result<(), std::io::Error>> {
  async move {
    let mut entries = fs::read_dir(path).await?;
    while let Some(entry) = entries.next_entry().await? {
      let path = entry.path();
      if path.is_dir() {
        find_test_files(&path, test_files).await?;
      } else if path.is_file() && path.to_string_lossy().ends_with("_test.km") {
        test_files.push(path);
      }
    }
    Ok(())
  }
  .boxed()
}

impl Test {
  pub async fn execute(&self, root_path: &str) -> Result<()> {
    let mut passed = 0;
    let mut failed = 0;

    let mut test_files = Vec::new();

    let mut path = Path::new(root_path);
    let mut path_buf = path.to_path_buf();
    path_buf.push(&self.path);
    path = path_buf.as_path();

    find_test_files(path, &mut test_files).await?;

    for path_buf in test_files {
      let path = path_buf.as_path();
      let arena = Bump::new();
      let content = fs::read_to_string(&path).await?;
      let interpreter = Interpreter::new(&arena, Some(path.to_str().unwrap()));
      let content_in_arena = arena.alloc_str(&content);

      println!("Running {}", path.display());

      match interpreter.run_tests(content_in_arena).await {
        Ok(results) => {
          for result in results {
            if result.passed {
              println!("  test {} ... ok", result.name);
              passed += 1;
            } else {
              println!("  '{}'... failed", result.name);
              if let Some(err) = result.error {
                report_error(path.to_str().unwrap(), &content, err);
              }
              failed += 1;
            }
          }
        }
        Err(err) => {
          report_error(path.to_str().unwrap(), &content, err);
          failed += 1;
        }
      }
    }
    println!("\nTest results: {} passed, {} failed", passed, failed);
    Ok(())
  }
}
