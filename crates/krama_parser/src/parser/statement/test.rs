use krama_core::{
  ast::{
    precedence::Precedence,
    statement::{Statement, StatementKind},
  },
  error::ErrorKind,
};

use crate::parser::Parser;

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub(super) fn parse_test_statement(
    &mut self,
  ) -> Result<Statement<'ast>, ErrorKind> {
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
