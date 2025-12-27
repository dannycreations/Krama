use krama_core::{ErrorKindResult, PrecedenceKind, Statement, StatementKind};

use crate::Parser;

impl<'a> Parser<'a> {
  pub fn parse_test_statement(&mut self) -> ErrorKindResult<Statement> {
    let start_span = self.current_token.span;
    self.advance();
    let name = self.parse_expression(PrecedenceKind::Lowest)?;
    let body = self.parse_block_statement()?;
    Ok(Statement::new(
      StatementKind::Test {
        name: Box::new(name),
        body: Box::new(body),
      },
      start_span,
    ))
  }
}
