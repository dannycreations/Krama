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
      let key = match self.current_token.kind {
        TokenKind::Identifier(_) => {
          let key_span = self.current_token.span;
          let key_ident = self.parse_identifier()?;
          Expression::new(
            ExpressionKind::Literal(Literal::String(key_ident)),
            key_span,
          )
        }
        TokenKind::String(_) => self.parse_literal()?,
        _ => {
          return Err(ErrorKind::SyntaxError(
            "Expected string for object key".to_string(),
          ));
        }
      };
      self.consume(TokenKind::Colon)?;
      let value = self.parse_expression(Precedence::Lowest)?;
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
