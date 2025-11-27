use bumpalo::collections::Vec as BumpVec;
use krama_core::{
  error::{Error, ErrorKind},
  token::TokenKind,
};

use super::{ParseError, Parser, Precedence};

impl<'a, 'ast> Parser<'a, 'ast> {
  pub(super) fn parse_paren_expression(&mut self) -> ParseError<'ast> {
    let start_span = self.current_token.span;
    self.advance();

    if self.current_token.kind == TokenKind::RParen {
      self.advance();
      return self.parse_fn_expr_with_empty_params(start_span);
    }

    let mut expressions = BumpVec::new_in(self.arena);
    expressions.push(self.parse_expression(Precedence::Lowest)?);

    while self.current_token.kind == TokenKind::Comma {
      self.advance();
      expressions.push(self.parse_expression(Precedence::Lowest)?);
    }

    self.consume_token(TokenKind::RParen)?;

    if self.current_token.kind == TokenKind::Arrow
      || self.current_token.kind == TokenKind::LBrace
    {
      let parameters = self.parse_fn_parameters()?;
      self.parse_fn_expr_with_params(start_span, parameters)
    } else if expressions.len() == 1 {
      expressions.pop().ok_or_else(|| Error {
        span: start_span,
        kind: ErrorKind::SyntaxError(
          "Expected expression in parentheses".to_string(),
        ),
      })
    } else {
      Err(Error {
                span: start_span,
                kind: ErrorKind::SyntaxError("Invalid grouped expression. It should contain only one expression. Tuples are not supported.".to_string()),
            })
    }
  }
}
