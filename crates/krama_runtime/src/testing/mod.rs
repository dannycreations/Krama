mod macros;

use krama_core::{Error, ExpressionKind, Statement, StatementKind};

use crate::Interpreter;

#[derive(Debug, Clone, PartialEq)]
pub enum TestResult {
  Success(String),
  Failure(String, Error),
}

impl Interpreter {
  pub async fn run_tests(&self, statements: &[Statement]) -> Vec<TestResult> {
    let test_futures = statements
      .iter()
      .filter_map(|statement| {
        if let StatementKind::Test { name, .. } = &statement.kind {
          let test_name = if let ExpressionKind::Literal(literal) = &name.kind {
            literal.to_string()
          } else {
            "".to_string()
          };
          Some((test_name, self.eval_statement(statement)))
        } else {
          None
        }
      })
      .collect::<Vec<_>>();

    let mut results = Vec::new();
    for (name, future) in test_futures {
      let result = future.await;
      results.push(match result {
        Ok(_) => TestResult::Success(name),
        Err(e) => TestResult::Failure(name, e),
      });
    }
    results
  }
}
