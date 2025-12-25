use krama_core::{ErrorKind, Expression, ExpressionKind, Token, TokenKind};

use super::{ParseResult, Parser};

impl<'a> Parser<'a> {
  pub fn parse_import_expression(&mut self) -> ParseResult {
    let start_span = self.current_token.span;
    self.advance();

    if self.current_token.kind != TokenKind::LParen {
      return Err(ErrorKind::SyntaxError(
        "Expected '(' after 'import'".to_string(),
      ));
    }
    self.advance();

    let path = match self.current_token.clone() {
      Token {
        kind: TokenKind::String(path),
        ..
      } => path.clone(),
      _ => {
        return Err(ErrorKind::SyntaxError(
          "Expected a string literal for the import path".to_string(),
        ))
      }
    };
    self.advance();

    if self.current_token.kind != TokenKind::RParen {
      return Err(ErrorKind::SyntaxError(
        "Expected ')' after import path".to_string(),
      ));
    }
    self.advance();

    Ok(Expression::new(
      ExpressionKind::Import { path, items: None },
      start_span,
    ))
  }
}
