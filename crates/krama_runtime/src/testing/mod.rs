pub mod macros;

use krama_core::{
  ast::{
    expression::ExpressionKind,
    statement::{Statement, StatementKind},
  },
  error::Error,
};

use crate::interpreter::Interpreter;

#[derive(Debug, Clone, PartialEq)]
pub enum TestResult<'ast> {
  Success(String),
  Failure(String, Error<'ast>),
}

impl<'ast> Interpreter<'ast> {
  pub async fn run_tests(
    &self,
    statements: &[Statement<'ast>],
  ) -> Vec<TestResult<'ast>> {
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
