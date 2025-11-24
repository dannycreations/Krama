use super::ParseError;
use super::Parser;
use krama_core::ast::expression::Expression;
use krama_core::ast::expression::ExpressionKind;
use krama_core::ast::operator::BinaryOperator;
use krama_core::ast::operator::UpdateOperator;
use krama_core::error::Error;
use krama_core::error::ErrorKind;
use krama_core::span::Span;
use krama_core::token::TokenKind;

impl<'a, 'ast> Parser<'a, 'ast>
where
  'a: 'ast,
{
  pub(super) fn parse_postfix_expression(
    &mut self,
    argument: Expression<'ast>,
  ) -> ParseError<'ast> {
    let token = *self.current_token.as_ref().unwrap();
    let operator = match token.kind {
      TokenKind::PlusPlus => UpdateOperator::Increment,
      TokenKind::MinusMinus => UpdateOperator::Decrement,
      _ => {
        return Err(Error {
          span: token.span,
          kind: ErrorKind::ParserError("Invalid postfix operator"),
        })
      }
    };
    self.advance();

    Ok(Expression {
      kind: ExpressionKind::Update {
        operator,
        argument: self.arena.alloc(argument),
        prefix: false,
      },
      span: token.span,
    })
  }

  pub(super) fn parse_infix_expression(
    &mut self,
    left: Expression<'ast>,
  ) -> ParseError<'ast> {
    let precedence = self.current_precedence();
    let token = *self.current_token.as_ref().unwrap();

    let operator = match token.kind {
      TokenKind::Plus => BinaryOperator::Add,
      TokenKind::Minus => BinaryOperator::Subtract,
      TokenKind::Star => {
        if self
          .peek_token
          .as_ref()
          .is_some_and(|t| t.kind == TokenKind::Star)
        {
          self.advance();
          BinaryOperator::Exponent
        } else {
          BinaryOperator::Multiply
        }
      }
      TokenKind::Slash => BinaryOperator::Divide,
      TokenKind::Percent => BinaryOperator::Modulo,
      TokenKind::Equal => BinaryOperator::Assign,
      TokenKind::EqualEqual => BinaryOperator::Equal,
      TokenKind::BangEqual => BinaryOperator::NotEqual,
      TokenKind::LessThan => BinaryOperator::LessThan,
      TokenKind::LessThanEqual => BinaryOperator::LessThanOrEqual,
      TokenKind::GreaterThan => BinaryOperator::GreaterThan,
      TokenKind::GreaterThanEqual => BinaryOperator::GreaterThanOrEqual,
      TokenKind::PlusEqual => BinaryOperator::Add,
      TokenKind::MinusEqual => BinaryOperator::Subtract,
      TokenKind::StarEqual => BinaryOperator::Multiply,
      TokenKind::SlashEqual => BinaryOperator::Divide,
      TokenKind::PercentEqual => BinaryOperator::Modulo,
      TokenKind::AmpersandAmpersand => BinaryOperator::LogicalAnd,
      TokenKind::PipePipe => BinaryOperator::LogicalOr,
      TokenKind::Ampersand => BinaryOperator::BitwiseAnd,
      TokenKind::Pipe => BinaryOperator::BitwiseOr,
      TokenKind::Caret => BinaryOperator::BitwiseXor,
      TokenKind::LessLess => BinaryOperator::LeftShift,
      TokenKind::GreaterGreater => BinaryOperator::RightShift,
      TokenKind::AmpersandEqual => BinaryOperator::BitwiseAnd,
      TokenKind::PipeEqual => BinaryOperator::BitwiseOr,
      TokenKind::CaretEqual => BinaryOperator::BitwiseXor,
      TokenKind::LessLessEqual => BinaryOperator::LeftShift,
      TokenKind::GreaterGreaterEqual => BinaryOperator::RightShift,
      _ => {
        return Err(Error {
          span: token.span,
          kind: ErrorKind::ParserError("Invalid infix operator"),
        })
      }
    };

    self.advance();
    let right = self.parse_expression(precedence)?;

    if token.kind == TokenKind::Equal
      || token.kind == TokenKind::PlusEqual
      || token.kind == TokenKind::MinusEqual
      || token.kind == TokenKind::StarEqual
      || token.kind == TokenKind::SlashEqual
      || token.kind == TokenKind::PercentEqual
      || token.kind == TokenKind::AmpersandEqual
      || token.kind == TokenKind::PipeEqual
      || token.kind == TokenKind::CaretEqual
      || token.kind == TokenKind::LessLessEqual
      || token.kind == TokenKind::GreaterGreaterEqual
    {
      Ok(Expression {
        kind: ExpressionKind::Assignment {
          left: self.arena.alloc(left),
          operator,
          right: self.arena.alloc(right),
        },
        span: token.span,
      })
    } else {
      let span = Span::new(left.span.start, right.span.end);
      Ok(Expression {
        kind: ExpressionKind::Binary {
          left: self.arena.alloc(left),
          operator,
          right: self.arena.alloc(right),
        },
        span,
      })
    }
  }
}
