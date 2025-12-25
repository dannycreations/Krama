use krama_core::{Expression, ExpressionKind, PrecedenceKind, TokenKind};

use super::{ParseResult, Parser};

impl<'a> Parser<'a> {
  pub fn parse_index_expression(&mut self, left: Expression) -> ParseResult {
    self.advance();

    let index = self.parse_expression(PrecedenceKind::Lowest)?;

    self.consume(TokenKind::RBracket)?;
    let span = left.span.merge(&self.current_token.span);
    Ok(Expression::new(
      ExpressionKind::Index {
        object: Box::new(left),
        index: Box::new(index),
      },
      span,
    ))
  }
}
