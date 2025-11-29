use krama_core::{
  ast::{
    expression::{Expression, ExpressionKind},
    literal::Literal,
  },
  error::ErrorKind,
  token::TokenKind,
};

use crate::parser::{ParseError, Parser};

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub(super) fn parse_identifier_expression(&mut self) -> ParseError<'a, 'ast> {
    let token = self.current_token.clone();
    let name = self.parse_identifier()?;
    Ok(Expression::new(
      ExpressionKind::Identifier(name),
      token.span,
    ))
  }

  pub(super) fn parse_integer(&mut self) -> ParseError<'a, 'ast> {
    let token = self.current_token.clone();
    if let TokenKind::Integer(value) = token.kind {
      self.advance();
      let value: i64 = value.replace('_', "").parse().map_err(|_| {
        (
          ErrorKind::SyntaxError("Invalid integer literal".to_string()),
          token.span.clone(),
        )
      })?;
      Ok(Expression::new(
        ExpressionKind::Literal(Literal::Integer(value)),
        token.span,
      ))
    } else {
      Err((
        ErrorKind::SyntaxError("Expected integer".to_string()),
        token.span,
      ))
    }
  }

  pub(super) fn parse_float(&mut self) -> ParseError<'a, 'ast> {
    let token = self.current_token.clone();
    if let TokenKind::Float(value) = token.kind {
      self.advance();
      let value: f64 = value.replace('_', "").parse().map_err(|_| {
        (
          ErrorKind::SyntaxError("Invalid float literal".to_string()),
          token.span.clone(),
        )
      })?;
      Ok(Expression::new(
        ExpressionKind::Literal(Literal::Float(value)),
        token.span,
      ))
    } else {
      Err((
        ErrorKind::SyntaxError("Expected float".to_string()),
        token.span,
      ))
    }
  }

  pub(super) fn parse_string(&mut self) -> ParseError<'a, 'ast> {
    let token = self.current_token.clone();
    if let TokenKind::String(value) = token.kind {
      self.advance();
      Ok(Expression::new(
        ExpressionKind::Literal(Literal::String(self.arena.alloc_str(value))),
        token.span,
      ))
    } else {
      Err((
        ErrorKind::SyntaxError("Expected string".to_string()),
        token.span,
      ))
    }
  }

  pub(super) fn parse_boolean(&mut self) -> ParseError<'a, 'ast> {
    let token = self.current_token.clone();
    self.advance();
    let value = token.kind == TokenKind::True;
    Ok(Expression::new(
      ExpressionKind::Literal(Literal::Boolean(value)),
      token.span,
    ))
  }

  pub(super) fn parse_null(&mut self) -> ParseError<'a, 'ast> {
    let token = self.current_token.clone();
    self.advance();
    Ok(Expression::new(
      ExpressionKind::Literal(Literal::Null),
      token.span,
    ))
  }
}
