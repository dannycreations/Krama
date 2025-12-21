use bumpalo::collections::Vec as BumpVec;
use krama_core::{
  ErrorKind, Expression, ExpressionKind, Literal, Precedence, TokenKind,
};

use super::{ParseResult, Parser};

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub fn parse_object_expression(&mut self) -> ParseResult<'a, 'ast> {
    let start_span = self.consume(TokenKind::LBrace)?.span;

    let mut properties = BumpVec::new_in(self.arena);

    if self.current_token.kind == TokenKind::RBrace {
      let end_span = self.current_token.span;
      self.advance();
      return Ok(Expression::new(
        ExpressionKind::Object { properties },
        start_span.merge(&end_span),
      ));
    }

    loop {
      let (key, value) = match self.current_token.kind {
        TokenKind::Identifier(_) => {
          let key_span = self.current_token.span;
          let key_ident = self.parse_identifier()?;
          let key_expr = Expression::new(
            ExpressionKind::Literal(Literal::String(key_ident)),
            key_span,
          );

          if self.current_token.kind == TokenKind::Colon {
            self.advance();
            let value = self.parse_expression(Precedence::Lowest)?;
            (key_expr, value)
          } else {
            // Shorthand: { x } -> { x: x }
            let value =
              Expression::new(ExpressionKind::Identifier(key_ident), key_span);
            (key_expr, value)
          }
        }
        TokenKind::String(_) => {
          let key = self.parse_literal()?;
          self.consume(TokenKind::Colon)?;
          let value = self.parse_expression(Precedence::Lowest)?;
          (key, value)
        }
        _ => {
          return Err(ErrorKind::SyntaxError(
            "Expected string or identifier for object key".to_string(),
          ));
        }
      };

      properties.push((key, value));

      if self.current_token.kind == TokenKind::RBrace {
        break;
      }
      self.consume(TokenKind::Comma)?;
      if self.current_token.kind == TokenKind::RBrace {
        break;
      }
    }

    let end_span = self.consume(TokenKind::RBrace)?.span;

    Ok(Expression::new(
      ExpressionKind::Object { properties },
      start_span.merge(&end_span),
    ))
  }
}
