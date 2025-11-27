use krama_core::{
  ast::statement::{Statement, StatementKind},
  error::Error,
  token::TokenKind,
};

use super::{super::precedence::Precedence, Parser};

impl<'a, 'ast> Parser<'a, 'ast>
where
  'a: 'ast,
{
  pub(super) fn parse_return_statement(
    &mut self,
  ) -> Result<Statement<'ast>, Error> {
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
        value: value.map(|v| self.arena.alloc(v) as &_),
      },
      start_span,
    ))
  }

  pub(super) fn parse_break_statement(
    &mut self,
  ) -> Result<Statement<'ast>, Error> {
    let start_span = self.current_token.span;
    self.advance();
    Ok(Statement::new(StatementKind::Break, start_span))
  }

  pub(super) fn parse_continue_statement(
    &mut self,
  ) -> Result<Statement<'ast>, Error> {
    let start_span = self.current_token.span;
    self.advance();
    Ok(Statement::new(StatementKind::Continue, start_span))
  }
}
