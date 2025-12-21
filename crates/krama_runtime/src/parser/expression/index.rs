use krama_core::{Expression, ExpressionKind, Precedence, TokenKind};

use super::{ParseResult, Parser};

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub fn parse_index_expression(
    &mut self,
    left: Expression<'ast>,
  ) -> ParseResult<'a, 'ast> {
    self.advance();

    let index = self.parse_expression(Precedence::Lowest)?;

    self.consume(TokenKind::RBracket)?;
    let span = left.span.merge(&self.current_token.span);
    Ok(Expression::new(
      ExpressionKind::Index {
        object: self.arena.alloc(left),
        index: self.arena.alloc(index),
      },
      span,
    ))
  }
}
