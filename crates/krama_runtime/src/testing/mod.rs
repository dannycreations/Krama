use krama_core::{
  ast::{expression::ExpressionKind, statement::StatementKind},
  error::Error,
};

use crate::interpreter::Interpreter;

#[derive(Debug, Clone, PartialEq)]
pub enum TestResult {
  Success(String),
  Failure(String, Error),
}

impl<'ast> Interpreter<'ast> {
  pub async fn run_tests(&self) -> Vec<TestResult> {
    let program = match self.parse_and_resolve("") {
      Ok(program) => program,
      Err(e) => return vec![TestResult::Failure("".to_string(), e)],
    };

    let test_futures = program
      .statements
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
