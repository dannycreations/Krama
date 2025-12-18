use krama_core::{ErrorKind, Precedence, Statement, StatementKind};

use super::Parser;

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub fn parse_test_statement(&mut self) -> Result<Statement<'ast>, ErrorKind> {
    let start_span = self.current_token.span.clone();
    self.advance();
    let name = self.parse_expression(Precedence::Lowest)?;
    let body = self.parse_block_statement()?;
    Ok(Statement::new(
      StatementKind::Test {
        name: self.arena.alloc(name),
        body: self.arena.alloc(body),
      },
      start_span,
    ))
  }
}
