use krama_core::{
  ast::expression::{Expression, ExpressionKind},
  token::TokenKind,
};

use crate::parser::{precedence::Precedence, ParseError, Parser};

impl<'a, 'ast> Parser<'a, 'ast>
where
  'a: 'ast,
{
  pub(super) fn parse_index_expression(
    &mut self,
    left: Expression<'ast>,
  ) -> ParseError<'ast> {
    self.advance();

    let index = self.parse_expression(Precedence::Lowest)?;

    self.consume_token(TokenKind::RBracket)?;
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
