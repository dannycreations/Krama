use krama_core::{
  ast::statement::{Statement, StatementKind},
  error::{Error, ErrorKind},
  span::Span,
  token::TokenKind,
};

use super::Parser;

impl<'a, 'ast> Parser<'a, 'ast>
where
  'a: 'ast,
{
  pub(super) fn parse_fn_statement(
    &mut self,
    public: bool,
    start_span: Span,
  ) -> Result<Statement<'ast>, Error> {
    self.advance();
    let name = if let TokenKind::Identifier(name) = self.current_token.kind {
      self.arena.alloc_str(name)
    } else {
      return Err(Error {
        span: start_span,
        kind: ErrorKind::SyntaxError(
          "Expected function name after 'fn'".to_string(),
        ),
      });
    };
    self.advance();
    self.consume_token(TokenKind::LParen)?;
    let parameters = self.parse_fn_parameters()?;
    let kind = if self.current_token.kind == TokenKind::Colon {
      self.advance();
      Some(self.parse_type()?)
    } else {
      None
    };

    let body = self.parse_block_statement()?;
    Ok(Statement::new(
      StatementKind::Fn {
        public,
        name,
        parameters,
        body: self.arena.alloc(body),
        kind,
      },
      start_span,
    ))
  }
}
