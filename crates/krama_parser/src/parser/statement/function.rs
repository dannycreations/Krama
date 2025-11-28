use krama_core::{
  ast::{
    expression::FunctionBody,
    precedence::Precedence,
    statement::{Statement, StatementKind},
  },
  error::{Error, ErrorKind},
  span::Span,
  token::TokenKind,
};

use super::Parser;

impl<'a, 'ast> Parser<'a, 'ast> {
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

    let body = if self.current_token.kind == TokenKind::LBrace {
      let block = self.parse_block_statement()?;
      FunctionBody::Block(self.arena.alloc(block))
    } else if self.current_token.kind == TokenKind::Equal {
      self.advance();
      let expr = self.parse_expression(Precedence::Lowest)?;
      FunctionBody::Expression(self.arena.alloc(expr))
    } else {
      return Err(Error {
        span: self.current_token.span,
        kind: ErrorKind::SyntaxError("Expected function body".to_string()),
      });
    };

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
