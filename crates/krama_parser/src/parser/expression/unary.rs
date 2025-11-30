use krama_core::{
  ast::{
    expression::{Expression, ExpressionKind},
    operator::{UnaryOperator, UpdateOperator},
    precedence::Precedence,
  },
  error::ErrorKind,
  token::TokenKind,
};

use crate::parser::{ParseResult, Parser};

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub(super) fn parse_unary_expression(&mut self) -> ParseResult<'a, 'ast> {
    let token = self.current_token.clone();
    self.advance();
    let operator = match token.kind {
      TokenKind::Bang => UnaryOperator::Not,
      TokenKind::Minus => UnaryOperator::Negate,
      TokenKind::Tilde => UnaryOperator::BitwiseNot,
      TokenKind::Plus => return self.parse_expression(Precedence::Prefix),
      _ => {
        return Err((
          ErrorKind::SyntaxError("Invalid unary operator".to_string()),
          token.span,
        ))
      }
    };
    let right = self.parse_expression(Precedence::Prefix)?;
    Ok(Expression::new(
      ExpressionKind::Unary {
        operator,
        right: self.arena.alloc(right),
      },
      token.span,
    ))
  }

  pub(super) fn parse_prefix_update_expression(
    &mut self,
  ) -> ParseResult<'a, 'ast> {
    let token = self.current_token.clone();
    self.advance();
    let operator = match token.kind {
      TokenKind::PlusPlus => UpdateOperator::Increment,
      TokenKind::MinusMinus => UpdateOperator::Decrement,
      _ => {
        return Err((
          ErrorKind::SyntaxError("Invalid prefix operator".to_string()),
          token.span,
        ))
      }
    };
    let argument = self.parse_expression(Precedence::Prefix)?;

    Ok(Expression::new(
      ExpressionKind::Update {
        operator,
        argument: self.arena.alloc(argument),
        prefix: true,
      },
      token.span,
    ))
  }
}
