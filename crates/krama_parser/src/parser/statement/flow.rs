use krama_core::{
  ast::{
    precedence::Precedence,
    statement::{Statement, StatementKind},
  },
  error::ErrorKind,
  span::Span,
  token::TokenKind,
};

use crate::parser::Parser;

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub(super) fn parse_return_statement(
    &mut self,
  ) -> Result<Statement<'ast>, (ErrorKind, Span<'a>)> {
    let start_span = self.current_token.span.clone();
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

  pub(super) fn parse_break_statement(
    &mut self,
  ) -> Result<Statement<'ast>, (ErrorKind, Span<'a>)> {
    let start_span = self.current_token.span.clone();
    self.advance();
    Ok(Statement::new(StatementKind::Break, start_span))
  }

  pub(super) fn parse_continue_statement(
    &mut self,
  ) -> Result<Statement<'ast>, (ErrorKind, Span<'a>)> {
    let start_span = self.current_token.span.clone();
    self.advance();
    Ok(Statement::new(StatementKind::Continue, start_span))
  }
}
