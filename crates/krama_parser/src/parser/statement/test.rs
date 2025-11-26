use krama_core::{
  ast::statement::{Statement, StatementKind},
  error::Error,
};

use super::{super::precedence::Precedence, Parser};

impl<'a, 'ast> Parser<'a, 'ast>
where
  'a: 'ast,
{
  pub(super) fn parse_test_statement(
    &mut self,
  ) -> Result<Statement<'ast>, Error> {
    let start_span = self.current_token.span;
    self.advance();
    let name = self.parse_expression(Precedence::Lowest)?;
    let body = self.parse_block_statement()?;
    Ok(Statement {
      kind: StatementKind::Test {
        name: self.arena.alloc(name),
        body: self.arena.alloc(body),
      },
      span: start_span,
    })
  }
}
