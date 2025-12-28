use krama_core::{ErrorKind, Expression, ExpressionKind, Literal, TokenKind};

use crate::{ParseResult, Parser};

impl<'a> Parser<'a> {
  /// Parses a literal expression (Integer, Float, String, Boolean, Null).
  pub fn parse_literal(&mut self) -> ParseResult {
    let token = self.current_token.clone();
    self.advance();

    let literal = match token.kind {
      TokenKind::Integer(value) => {
        Literal::Integer(value.replace('_', "").parse().map_err(|_| {
          ErrorKind::SyntaxError("Invalid integer literal".to_string())
        })?)
      }
      TokenKind::Float(value) => {
        Literal::Float(value.replace('_', "").parse().map_err(|_| {
          ErrorKind::SyntaxError("Invalid float literal".to_string())
        })?)
      }
      TokenKind::String(value) => Literal::String(value.clone()),
      TokenKind::True => Literal::Bool(true),
      TokenKind::False => Literal::Bool(false),
      TokenKind::Null => Literal::Null,
      _ => {
        return Err(ErrorKind::SyntaxError(format!(
          "Unexpected token for literal: {}",
          token.kind
        )))
      }
    };

    Ok(Expression::new(
      ExpressionKind::Literal(literal),
      token.span,
    ))
  }
}
