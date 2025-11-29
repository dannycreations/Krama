use bumpalo::collections::Vec as BumpVec;
use krama_core::{
  ast::{
    expression::{Expression, ExpressionKind},
    precedence::Precedence,
  },
  token::TokenKind,
};

use crate::parser::{ParseError, Parser};

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub(super) fn parse_collection_expression(&mut self) -> ParseError<'a, 'ast> {
    let start_span = self.current_token.span.clone();

    let mut elements = BumpVec::new_in(self.arena);

    // Check for empty collection
    if self
      .lexer
      .peek()
      .is_some_and(|t| t.kind == TokenKind::RBracket)
    {
      self.advance();
      let end_span = self.current_token.span.clone();
      self.advance();
      return Ok(Expression::new(
        ExpressionKind::Collection { elements },
        start_span.merge(&end_span),
      ));
    }
    self.advance();

    // Parse expressions
    loop {
      elements.push(self.parse_expression(Precedence::Lowest)?);
      if self.current_token.kind == TokenKind::RBracket {
        break;
      }
      self.consume_token(TokenKind::Comma)?;
      if self.current_token.kind == TokenKind::RBracket {
        // Allow trailing comma
        break;
      }
    }

    let end_span = self.current_token.span.clone();
    self.consume_token(TokenKind::RBracket)?;

    Ok(Expression::new(
      ExpressionKind::Collection { elements },
      start_span.merge(&end_span),
    ))
  }
}
