use krama_core::{ErrorKind, Precedence, Statement, StatementKind, TokenKind};

use super::Parser;

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub fn parse_return_statement(
    &mut self,
  ) -> Result<Statement<'ast>, ErrorKind> {
    let start_span = self.current_token.span;
    self.advance();

    let value = if !matches!(
      self.current_token.kind,
      TokenKind::Semicolon
        | TokenKind::Newline
        | TokenKind::RBrace
        | TokenKind::Eof
    ) {
      Some(self.parse_expression(Precedence::Lowest)?)
    } else {
      None
    };

    Ok(Statement::new(
      StatementKind::Return {
        value: value.map(|v| &*self.arena.alloc(v)),
      },
      start_span,
    ))
  }

  pub fn parse_break_statement(
    &mut self,
  ) -> Result<Statement<'ast>, ErrorKind> {
    let start_span = self.current_token.span;
    self.advance();
    Ok(Statement::new(StatementKind::Break, start_span))
  }

  pub fn parse_continue_statement(
    &mut self,
  ) -> Result<Statement<'ast>, ErrorKind> {
    let start_span = self.current_token.span;
    self.advance();
    Ok(Statement::new(StatementKind::Continue, start_span))
  }
}
