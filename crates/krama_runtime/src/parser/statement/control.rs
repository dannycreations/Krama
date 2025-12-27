use krama_core::{
  ErrorKindResult, PrecedenceKind, Statement, StatementKind, TokenKind,
};

use crate::Parser;

impl<'a> Parser<'a> {
  pub fn parse_return_statement(&mut self) -> ErrorKindResult<Statement> {
    let start_span = self.current_token.span;
    self.advance();

    let value = if !matches!(
      self.current_token.kind,
      TokenKind::Semicolon | TokenKind::RBrace | TokenKind::Eof
    ) {
      Some(self.parse_expression(PrecedenceKind::Lowest)?)
    } else {
      None
    };

    Ok(Statement::new(
      StatementKind::Return {
        value: value.map(Box::new),
      },
      start_span,
    ))
  }

  pub fn parse_break_statement(&mut self) -> ErrorKindResult<Statement> {
    let start_span = self.current_token.span;
    self.advance();
    Ok(Statement::new(StatementKind::Break, start_span))
  }

  pub fn parse_continue_statement(&mut self) -> ErrorKindResult<Statement> {
    let start_span = self.current_token.span;
    self.advance();
    Ok(Statement::new(StatementKind::Continue, start_span))
  }
}
