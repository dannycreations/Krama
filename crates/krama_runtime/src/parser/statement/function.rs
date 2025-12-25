use krama_core::{ErrorKindResult, Span, Statement, StatementKind, TokenKind};

use super::Parser;

impl<'a> Parser<'a> {
  /// Parses a function statement (`fn name(...) {...}`).
  pub fn parse_fn_statement(
    &mut self,
    public: bool,
    start_span: Span,
  ) -> ErrorKindResult<Statement> {
    self.advance();
    let name = self.parse_identifier()?;
    self.consume(TokenKind::LParen)?;

    // Reuse parse_fn_parameters for consistency.
    let parameters = self.parse_fn_parameters()?;
    self.consume(TokenKind::RParen)?;

    // Reuse parse_classic_fn_body_and_return_type for consistency.
    let (body, kind) = self.parse_classic_fn_body_and_return_type()?;

    Ok(Statement::new(
      StatementKind::Fn {
        public,
        name,
        parameters,
        body,
        kind,
      },
      start_span,
    ))
  }
}
