use std::{env, path::PathBuf};

use anyhow::{anyhow, Context, Result};
use bumpalo::Bump;
use clap::Parser;
use futures::future::join_all;
use krama_core::{error::ErrorKind, span::Span};
use krama_runtime::{interpreter::Interpreter, testing::TestResult};
use tokio::{fs, task};
use walkdir::WalkDir;

use crate::error::report_error;

#[derive(Parser)]
pub struct Test {
  #[clap(default_value = "src")]
  pub path: String,
}

#[derive(Debug)]
struct OwnedFailure {
  name: String,
  path: String,
  content: String,
  kind: ErrorKind,
  span: Span<'static>,
}

#[derive(Debug)]
enum TestKind {
  Success(String),
  Failure(OwnedFailure),
}

async fn find_test_files(path: PathBuf) -> Result<Vec<PathBuf>> {
  let found_files = task::spawn_blocking(move || {
    WalkDir::new(path)
      .into_iter()
      .filter_map(Result::ok)
      .filter(|e| {
        e.file_type().is_file()
          && e.path().to_string_lossy().ends_with("_test.km")
      })
      .map(|e| e.path().to_path_buf())
      .collect::<Vec<PathBuf>>()
  })
  .await
  .with_context(|| "Failed to search for test files")?;

  Ok(found_files)
}

impl Test {
  pub async fn execute(&self, _arena: &mut Bump) -> Result<()> {
    let mut passed = 0;
    let mut failed = 0;

    let root_path = env::current_dir()?;
    let mut path_buf = root_path;
    path_buf.push(&self.path);

    let test_files = find_test_files(path_buf).await?;

    let results = test_files.into_iter().map(|path_buf| async move {
      let arena = Bump::new();
      let path = path_buf.as_path();
      let content = fs::read_to_string(&path).await?;
      let path_str = path.to_str().context("path is not valid UTF-8")?;

      println!("Running tests in {}", path.display());

      let content_in_arena = arena.alloc_str(&content);
      let path_in_arena = arena.alloc_str(path_str);
      let interpreter = Interpreter::new(&arena, Some(path_in_arena));
      let program = interpreter
        .parse_and_resolve(content_in_arena)
        .map_err(|(kind, span)| anyhow!("Error: {}, Span: {:?}", kind, span))?;
      let results = interpreter.run_tests(&program.statements).await;

      let cli_results = results
        .into_iter()
        .map(|res| match res {
          TestResult::Success(name) => TestKind::Success(name),
          TestResult::Failure(name, (kind, span)) => {
            TestKind::Failure(OwnedFailure {
              name,
              path: path_str.to_string(),
              content: content.to_string(),
              kind,
              span: span.into_static(),
            })
          }
        })
        .collect::<Vec<_>>();
      Ok::<_, anyhow::Error>(cli_results)
    });

    let all_results = join_all(results).await;

    for result in all_results {
      match result {
        Ok(results) => {
          for result in results {
            match result {
              TestKind::Success(name) => {
                println!("  test {} ... ok", name);
                passed += 1;
              }
              TestKind::Failure(failure) => {
                println!("  '{}'... failed", failure.name);
                report_error(
                  &failure.path,
                  &failure.content,
                  failure.span,
                  failure.kind,
                );
                failed += 1;
              }
            }
          }
        }
        Err(e) => {
          eprintln!("Error running test file: {}", e);
          failed += 1;
        }
      }
    }

    println!("\nTest results: {} passed, {} failed", passed, failed);
    Ok(())
  }
}
