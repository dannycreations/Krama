use krama_core::{
  ErrorKind, Expression, ExpressionKind, LiteralKind, TokenKind,
};

use crate::{ParseResult, Parser};

impl<'a> Parser<'a> {
  /// Parses a literal expression (Integer, Float, String, Boolean, Null).
  pub fn parse_literal(&mut self) -> ParseResult {
    let token = self.current_token.clone();
    self.advance();

    let literal = match token.kind {
      TokenKind::Integer(value) => {
        LiteralKind::Integer(value.replace('_', "").parse().map_err(|_| {
          ErrorKind::SyntaxError("Invalid integer literal".to_string())
        })?)
      }
      TokenKind::Float(value) => {
        LiteralKind::Float(value.replace('_', "").parse().map_err(|_| {
          ErrorKind::SyntaxError("Invalid float literal".to_string())
        })?)
      }
      TokenKind::String(value) => LiteralKind::String(value.clone()),
      TokenKind::True => LiteralKind::Boolean(true),
      TokenKind::False => LiteralKind::Boolean(false),
      TokenKind::Null => LiteralKind::Null,
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
