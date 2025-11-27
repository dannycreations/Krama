use krama_core::{
  ast::{
    expression::{Expression, ExpressionKind},
    operator::{BinaryOperator, UpdateOperator},
  },
  error::{Error, ErrorKind},
  span::Span,
  token::TokenKind,
};

use super::{ParseError, Parser};

impl<'a, 'ast> Parser<'a, 'ast>
where
  'a: 'ast,
{
  pub(super) fn parse_postfix_expression(
    &mut self,
    argument: Expression<'ast>,
  ) -> ParseError<'ast> {
    let token = self.current_token;
    let operator = match token.kind {
      TokenKind::PlusPlus => UpdateOperator::Increment,
      TokenKind::MinusMinus => UpdateOperator::Decrement,
      _ => {
        return Err(Error {
          span: token.span,
          kind: ErrorKind::SyntaxError("Invalid postfix operator".to_string()),
        })
      }
    };
    self.advance();

    Ok(Expression::new(
      ExpressionKind::Update {
        operator,
        argument: self.arena.alloc(argument),
        prefix: false,
      },
      token.span,
    ))
  }

  pub(super) fn parse_infix_expression(
    &mut self,
    left: Expression<'ast>,
  ) -> ParseError<'ast> {
    let precedence = self.current_precedence();
    let token = self.current_token;

    let (operator, is_assignment) = match token.kind {
      TokenKind::Plus => (BinaryOperator::Add, false),
      TokenKind::Minus => (BinaryOperator::Subtract, false),
      TokenKind::Star => (BinaryOperator::Multiply, false),
      TokenKind::StarStar => (BinaryOperator::Exponent, false),
      TokenKind::Slash => (BinaryOperator::Divide, false),
      TokenKind::Percent => (BinaryOperator::Modulo, false),
      TokenKind::EqualEqual => (BinaryOperator::Equal, false),
      TokenKind::BangEqual => (BinaryOperator::NotEqual, false),
      TokenKind::LessThan => (BinaryOperator::LessThan, false),
      TokenKind::LessThanEqual => (BinaryOperator::LessThanOrEqual, false),
      TokenKind::GreaterThan => (BinaryOperator::GreaterThan, false),
      TokenKind::GreaterThanEqual => {
        (BinaryOperator::GreaterThanOrEqual, false)
      }
      TokenKind::AmpersandAmpersand => (BinaryOperator::LogicalAnd, false),
      TokenKind::PipePipe => (BinaryOperator::LogicalOr, false),
      TokenKind::Ampersand => (BinaryOperator::BitwiseAnd, false),
      TokenKind::Pipe => (BinaryOperator::BitwiseOr, false),
      TokenKind::Caret => (BinaryOperator::BitwiseXor, false),
      TokenKind::LessLess => (BinaryOperator::LeftShift, false),
      TokenKind::GreaterGreater => (BinaryOperator::RightShift, false),
      TokenKind::Equal => (BinaryOperator::Assign, true),
      TokenKind::PlusEqual => (BinaryOperator::Add, true),
      TokenKind::MinusEqual => (BinaryOperator::Subtract, true),
      TokenKind::StarEqual => (BinaryOperator::Multiply, true),
      TokenKind::SlashEqual => (BinaryOperator::Divide, true),
      TokenKind::PercentEqual => (BinaryOperator::Modulo, true),
      TokenKind::AmpersandEqual => (BinaryOperator::BitwiseAnd, true),
      TokenKind::PipeEqual => (BinaryOperator::BitwiseOr, true),
      TokenKind::CaretEqual => (BinaryOperator::BitwiseXor, true),
      TokenKind::LessLessEqual => (BinaryOperator::LeftShift, true),
      TokenKind::GreaterGreaterEqual => (BinaryOperator::RightShift, true),
      _ => {
        return Err(Error {
          span: token.span,
          kind: ErrorKind::SyntaxError("Invalid infix operator".to_string()),
        })
      }
    };

    self.advance();
    let right = self.parse_expression(precedence)?;

    if is_assignment {
      Ok(Expression::new(
        ExpressionKind::Assignment {
          left: self.arena.alloc(left),
          operator,
          right: self.arena.alloc(right),
        },
        token.span,
      ))
    } else {
      let span = Span::new(left.span.start, right.span.end);
      Ok(Expression::new(
        ExpressionKind::Binary {
          left: self.arena.alloc(left),
          operator,
          right: self.arena.alloc(right),
        },
        span,
      ))
    }
  }
}
