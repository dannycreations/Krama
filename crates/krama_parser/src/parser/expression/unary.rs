use krama_core::{
  ast::{
    expression::{Expression, ExpressionKind},
    operator::{UnaryOperator, UpdateOperator},
    precedence::Precedence,
  },
  error::{Error, ErrorKind},
  token::TokenKind,
};

use super::{ParseError, Parser};

impl<'a, 'ast> Parser<'a, 'ast> {
  pub(super) fn parse_unary_expression(&mut self) -> ParseError<'ast> {
    let token = self.current_token;
    self.advance();
    let operator = match token.kind {
      TokenKind::Bang => UnaryOperator::Not,
      TokenKind::Minus => UnaryOperator::Negate,
      TokenKind::Tilde => UnaryOperator::BitwiseNot,
      TokenKind::Plus => return self.parse_expression(Precedence::Prefix),
      _ => {
        return Err(Error {
          span: token.span,
          kind: ErrorKind::SyntaxError("Invalid unary operator".to_string()),
        })
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

  pub(super) fn parse_prefix_update_expression(&mut self) -> ParseError<'ast> {
    let token = self.current_token;
    self.advance();
    let operator = match token.kind {
      TokenKind::PlusPlus => UpdateOperator::Increment,
      TokenKind::MinusMinus => UpdateOperator::Decrement,
      _ => {
        return Err(Error {
          span: token.span,
          kind: ErrorKind::SyntaxError("Invalid prefix operator".to_string()),
        })
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
