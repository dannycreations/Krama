use super::Interpreter;
use futures::future::LocalBoxFuture;
use futures::FutureExt;
use krama_core::ast::expression::FunctionBody;
use krama_core::ast::statement::Statement;
use krama_core::ast::statement::StatementKind;
use krama_core::error::Error;
use krama_core::object::{Function, Object};
use std::rc::Rc;

pub struct TestResult {
  pub name: String,
  pub passed: bool,
  pub error: Option<Error>,
}

impl<'ast> Interpreter<'ast> {
  pub async fn run_tests(
    &self,
    source: &'ast str,
  ) -> Result<Vec<TestResult>, Error> {
    let lexer = krama_lexer::lexer::Lexer::new(source);
    let mut parser = krama_parser::parser::Parser::new(lexer, self.arena);
    let program = parser.parse()?;

    let mut results = Vec::new();
    let mut test_statements = Vec::new();

    self
      .process_statements_for_tests(&program.statements, &mut test_statements)
      .await?;
    self.execute_tests(&test_statements, &mut results).await?;

    Ok(results)
  }

  fn process_statements_for_tests<'s>(
    &'s self,
    statements: &'s [Statement<'ast>],
    test_statements: &'s mut Vec<Statement<'ast>>,
  ) -> LocalBoxFuture<'s, Result<(), Error>> {
    async move {
      if statements.is_empty() {
        return Ok(());
      }
      let (statement, rest) = statements.split_first().unwrap();
      if matches!(statement.kind, StatementKind::Test { .. }) {
        test_statements.push(statement.clone());
      } else {
        self.eval_statement(statement).await?;
      }
      self
        .process_statements_for_tests(rest, test_statements)
        .await
    }
    .boxed_local()
  }

  fn execute_tests<'s>(
    &'s self,
    test_statements: &'s [Statement<'ast>],
    results: &'s mut Vec<TestResult>,
  ) -> LocalBoxFuture<'s, Result<(), Error>> {
    async move {
      if test_statements.is_empty() {
        return Ok(());
      }
      let (statement, rest) = test_statements.split_first().unwrap();
      if let StatementKind::Test { name, body } = &statement.kind {
        let test_name = match self.eval_expression(name, None).await {
          Ok(Object::String(s)) => s.to_string(),
          _ => "Unnamed test".to_string(),
        };

        let function = krama_core::object::UserFn {
          parameters: bumpalo::collections::Vec::new_in(self.arena),
          body: FunctionBody::Block(body),
          kind: None,
        };
        let callee = Object::Function(Function::User(Rc::new(function)));

        let result = self
          .eval_call_expression(
            callee,
            bumpalo::collections::Vec::new_in(self.arena),
            statement.span,
          )
          .await;

        results.push(TestResult {
          name: test_name,
          passed: result.is_ok(),
          error: result.err(),
        });
      }
      self.execute_tests(rest, results).await
    }
    .boxed_local()
  }
}
