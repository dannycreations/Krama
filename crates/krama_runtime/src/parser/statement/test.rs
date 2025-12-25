use krama_core::{ErrorKind, PrecedenceKind, Statement, StatementKind};

use super::Parser;

impl<'a> Parser<'a> {
  pub fn parse_test_statement(&mut self) -> Result<Statement, ErrorKind> {
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
