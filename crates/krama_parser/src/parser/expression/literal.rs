use super::ParseError;
use super::Parser;
use krama_core::ast::expression::Expression;
use krama_core::ast::expression::ExpressionKind;
use krama_core::ast::literal::Literal;
use krama_core::error::Error;
use krama_core::error::ErrorKind;
use krama_core::token::TokenKind;

impl<'a, 'ast> Parser<'a, 'ast>
where
  'a: 'ast,
{
  pub(super) fn parse_identifier(&mut self) -> ParseError<'ast> {
    let token = *self.current_token.as_ref().unwrap();
    if let TokenKind::Identifier(name) = token.kind {
      self.advance();
      Ok(Expression {
        kind: ExpressionKind::Identifier(self.arena.alloc_str(name)),
        span: token.span,
      })
    } else {
      Err(Error {
        span: token.span,
        kind: ErrorKind::ParserError("Expected identifier"),
      })
    }
  }

  pub(super) fn parse_integer(&mut self) -> ParseError<'ast> {
    let token = *self.current_token.as_ref().unwrap();
    if let TokenKind::Integer(value) = token.kind {
      self.advance();
      let value: i64 = if value.contains('_') {
        value.replace('_', "").parse().unwrap()
      } else {
        value.parse().unwrap()
      };
      Ok(Expression {
        kind: ExpressionKind::Literal(Literal::Integer(value)),
        span: token.span,
      })
    } else {
      Err(Error {
        span: token.span,
        kind: ErrorKind::ParserError("Expected integer"),
      })
    }
  }

  pub(super) fn parse_float(&mut self) -> ParseError<'ast> {
    let token = *self.current_token.as_ref().unwrap();
    if let TokenKind::Float(value) = token.kind {
      self.advance();
      let value: f64 = if value.contains('_') {
        value.replace('_', "").parse().unwrap()
      } else {
        value.parse().unwrap()
      };
      Ok(Expression {
        kind: ExpressionKind::Literal(Literal::Float(value)),
        span: token.span,
      })
    } else {
      Err(Error {
        span: token.span,
        kind: ErrorKind::ParserError("Expected float"),
      })
    }
  }

  pub(super) fn parse_string(&mut self) -> ParseError<'ast> {
    let token = *self.current_token.as_ref().unwrap();
    if let TokenKind::String(value) = token.kind {
      self.advance();
      Ok(Expression {
        kind: ExpressionKind::Literal(Literal::String(
          self.arena.alloc_str(value),
        )),
        span: token.span,
      })
    } else {
      Err(Error {
        span: token.span,
        kind: ErrorKind::ParserError("Expected string"),
      })
    }
  }

  pub(super) fn parse_boolean(&mut self) -> ParseError<'ast> {
    let token = *self.current_token.as_ref().unwrap();
    self.advance();
    let value = token.kind == TokenKind::True;
    Ok(Expression {
      kind: ExpressionKind::Literal(Literal::Boolean(value)),
      span: token.span,
    })
  }

  pub(super) fn parse_null(&mut self) -> ParseError<'ast> {
    let token = *self.current_token.as_ref().unwrap();
    self.advance();
    Ok(Expression {
      kind: ExpressionKind::Literal(Literal::Null),
      span: token.span,
    })
  }
}
