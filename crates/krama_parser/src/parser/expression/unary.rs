use super::ParseError;
use super::Parser;
use super::Precedence;
use krama_core::ast::expression::Expression;
use krama_core::ast::expression::ExpressionKind;
use krama_core::ast::operator::UnaryOperator;
use krama_core::ast::operator::UpdateOperator;
use krama_core::error::Error;
use krama_core::error::ErrorKind;
use krama_core::token::TokenKind;

impl<'a, 'ast> Parser<'a, 'ast>
where
  'a: 'ast,
{
  pub(super) fn parse_unary_expression(&mut self) -> ParseError<'ast> {
    let token = *self.current_token.as_ref().unwrap();
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
    Ok(Expression {
      kind: ExpressionKind::Unary {
        operator,
        right: self.arena.alloc(right),
      },
      span: token.span,
    })
  }

  pub(super) fn parse_prefix_update_expression(&mut self) -> ParseError<'ast> {
    let token = *self.current_token.as_ref().unwrap();
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

    Ok(Expression {
      kind: ExpressionKind::Update {
        operator,
        argument: self.arena.alloc(argument),
        prefix: true,
      },
      span: token.span,
    })
  }
}
