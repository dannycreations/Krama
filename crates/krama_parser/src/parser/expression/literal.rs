use krama_core::{
  ast::{
    expression::{Expression, ExpressionKind},
    literal::Literal,
  },
  error::{Error, ErrorKind},
  token::TokenKind,
};

use super::{ParseError, Parser};

impl<'a, 'ast> Parser<'a, 'ast>
where
  'a: 'ast,
{
  pub(super) fn parse_identifier_expression(&mut self) -> ParseError<'ast> {
    let token = self.current_token;
    let name = self.parse_identifier()?;
    Ok(Expression::new(
      ExpressionKind::Identifier(name),
      token.span,
    ))
  }

  pub(super) fn parse_integer(&mut self) -> ParseError<'ast> {
    let token = self.current_token;
    if let TokenKind::Integer(value) = token.kind {
      self.advance();
      let value: i64 = value.replace('_', "").parse().map_err(|_| Error {
        span: token.span,
        kind: ErrorKind::SyntaxError("Invalid integer literal".to_string()),
      })?;
      Ok(Expression::new(
        ExpressionKind::Literal(Literal::Integer(value)),
        token.span,
      ))
    } else {
      Err(Error {
        span: token.span,
        kind: ErrorKind::SyntaxError("Expected integer".to_string()),
      })
    }
  }

  pub(super) fn parse_float(&mut self) -> ParseError<'ast> {
    let token = self.current_token;
    if let TokenKind::Float(value) = token.kind {
      self.advance();
      let value: f64 = value.replace('_', "").parse().map_err(|_| Error {
        span: token.span,
        kind: ErrorKind::SyntaxError("Invalid float literal".to_string()),
      })?;
      Ok(Expression::new(
        ExpressionKind::Literal(Literal::Float(value)),
        token.span,
      ))
    } else {
      Err(Error {
        span: token.span,
        kind: ErrorKind::SyntaxError("Expected float".to_string()),
      })
    }
  }

  pub(super) fn parse_string(&mut self) -> ParseError<'ast> {
    let token = self.current_token;
    if let TokenKind::String(value) = token.kind {
      self.advance();
      Ok(Expression::new(
        ExpressionKind::Literal(Literal::String(self.arena.alloc_str(value))),
        token.span,
      ))
    } else {
      Err(Error {
        span: token.span,
        kind: ErrorKind::SyntaxError("Expected string".to_string()),
      })
    }
  }

  pub(super) fn parse_boolean(&mut self) -> ParseError<'ast> {
    let token = self.current_token;
    self.advance();
    let value = token.kind == TokenKind::True;
    Ok(Expression::new(
      ExpressionKind::Literal(Literal::Boolean(value)),
      token.span,
    ))
  }

  pub(super) fn parse_null(&mut self) -> ParseError<'ast> {
    let token = self.current_token;
    self.advance();
    Ok(Expression::new(
      ExpressionKind::Literal(Literal::Null),
      token.span,
    ))
  }
}
