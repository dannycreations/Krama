use bumpalo::collections::Vec as BumpVec;
use krama_core::{
  ast::{
    expression::{Expression, ExpressionKind},
    literal::Literal,
    precedence::Precedence,
  },
  token::TokenKind,
};

use crate::parser::{ParseResult, Parser};

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub(super) fn parse_object_expression(&mut self) -> ParseResult<'a, 'ast> {
    let start_span = self.consume(TokenKind::LBrace)?.span;

    let mut properties = BumpVec::new_in(self.arena);

    if self.current_token.kind == TokenKind::RBrace {
      let end_span = self.current_token.span.clone();
      self.advance();
      return Ok(Expression::new(
        ExpressionKind::Object { properties },
        start_span.merge(&end_span),
      ));
    }

    // Parse properties
    loop {
      let key_span = self.current_token.span.clone();
      let key_ident = self.parse_identifier()?;
      let key = Expression::new(
        ExpressionKind::Literal(Literal::String(key_ident)),
        key_span,
      );
      self.consume(TokenKind::Colon)?;
      let value = self.parse_expression(Precedence::Lowest)?;
      properties.push((key, value));

      if self.current_token.kind == TokenKind::RBrace {
        break;
      }
      self.consume(TokenKind::Comma)?;
      if self.current_token.kind == TokenKind::RBrace {
        // Allow trailing comma
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
