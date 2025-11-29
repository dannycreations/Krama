use krama_core::{
  ast::{
    expression::{Expression, ExpressionKind},
    operator::{AssignmentOperator, BinaryOperator, UpdateOperator},
  },
  error::ErrorKind,
  token::TokenKind,
};

use crate::parser::{ParseError, Parser};

enum InfixOperator {
  Binary(BinaryOperator),
  Assignment(AssignmentOperator),
}

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub(super) fn parse_postfix_expression(
    &mut self,
    argument: Expression<'ast>,
  ) -> ParseError<'a, 'ast> {
    let token = self.current_token.clone();
    let operator = match token.kind {
      TokenKind::PlusPlus => UpdateOperator::Increment,
      TokenKind::MinusMinus => UpdateOperator::Decrement,
      _ => {
        return Err((
          ErrorKind::SyntaxError("Invalid postfix operator".to_string()),
          token.span,
        ))
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
  ) -> ParseError<'a, 'ast> {
    let precedence = self.current_precedence();
    let token = self.current_token.clone();

    let operator = match token.kind {
      TokenKind::Plus => InfixOperator::Binary(BinaryOperator::Add),
      TokenKind::Minus => InfixOperator::Binary(BinaryOperator::Subtract),
      TokenKind::Star => InfixOperator::Binary(BinaryOperator::Multiply),
      TokenKind::StarStar => InfixOperator::Binary(BinaryOperator::Exponent),
      TokenKind::Slash => InfixOperator::Binary(BinaryOperator::Divide),
      TokenKind::Percent => InfixOperator::Binary(BinaryOperator::Modulo),
      TokenKind::EqualEqual => InfixOperator::Binary(BinaryOperator::Equal),
      TokenKind::BangEqual => InfixOperator::Binary(BinaryOperator::NotEqual),
      TokenKind::LessThan => InfixOperator::Binary(BinaryOperator::LessThan),
      TokenKind::LessThanEqual => {
        InfixOperator::Binary(BinaryOperator::LessThanOrEqual)
      }
      TokenKind::GreaterThan => {
        InfixOperator::Binary(BinaryOperator::GreaterThan)
      }
      TokenKind::GreaterThanEqual => {
        InfixOperator::Binary(BinaryOperator::GreaterThanOrEqual)
      }
      TokenKind::AmpersandAmpersand => {
        InfixOperator::Binary(BinaryOperator::LogicalAnd)
      }
      TokenKind::PipePipe => InfixOperator::Binary(BinaryOperator::LogicalOr),
      TokenKind::Ampersand => InfixOperator::Binary(BinaryOperator::BitwiseAnd),
      TokenKind::Pipe => InfixOperator::Binary(BinaryOperator::BitwiseOr),
      TokenKind::Caret => InfixOperator::Binary(BinaryOperator::BitwiseXor),
      TokenKind::LessLess => InfixOperator::Binary(BinaryOperator::LeftShift),
      TokenKind::GreaterGreater => {
        InfixOperator::Binary(BinaryOperator::RightShift)
      }

      // Assignments
      TokenKind::Equal => InfixOperator::Assignment(AssignmentOperator::Assign),
      TokenKind::PlusEqual => {
        InfixOperator::Assignment(AssignmentOperator::AddAssign)
      }
      TokenKind::MinusEqual => {
        InfixOperator::Assignment(AssignmentOperator::SubtractAssign)
      }
      TokenKind::StarEqual => {
        InfixOperator::Assignment(AssignmentOperator::MultiplyAssign)
      }
      TokenKind::SlashEqual => {
        InfixOperator::Assignment(AssignmentOperator::DivideAssign)
      }
      TokenKind::PercentEqual => {
        InfixOperator::Assignment(AssignmentOperator::ModuloAssign)
      }
      TokenKind::AmpersandEqual => {
        InfixOperator::Assignment(AssignmentOperator::BitwiseAndAssign)
      }
      TokenKind::PipeEqual => {
        InfixOperator::Assignment(AssignmentOperator::BitwiseOrAssign)
      }
      TokenKind::CaretEqual => {
        InfixOperator::Assignment(AssignmentOperator::BitwiseXorAssign)
      }
      TokenKind::LessLessEqual => {
        InfixOperator::Assignment(AssignmentOperator::LeftShiftAssign)
      }
      TokenKind::GreaterGreaterEqual => {
        InfixOperator::Assignment(AssignmentOperator::RightShiftAssign)
      }
      _ => {
        return Err((
          ErrorKind::SyntaxError("Invalid infix operator".to_string()),
          token.span,
        ))
      }
    };

    self.advance();
    let right = self.parse_expression(precedence)?;

    match operator {
      InfixOperator::Assignment(op) => Ok(Expression::new(
        ExpressionKind::Assignment {
          left: self.arena.alloc(left),
          operator: op,
          right: self.arena.alloc(right),
        },
        token.span,
      )),
      InfixOperator::Binary(op) => {
        let span = left.span.merge(&right.span);
        Ok(Expression::new(
          ExpressionKind::Binary {
            left: self.arena.alloc(left),
            operator: op,
            right: self.arena.alloc(right),
          },
          span,
        ))
      }
    }
  }
}
