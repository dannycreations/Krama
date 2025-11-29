use krama_core::{
  ast::expression::{Expression, ExpressionKind},
  error::ErrorKind,
  token::{Token, TokenKind},
};

use crate::parser::{ParseError, Parser};

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub(super) fn parse_import_expression(&mut self) -> ParseError<'a, 'ast> {
    let start_span = self.current_token.span.clone();
    self.advance();

    if self.current_token.kind != TokenKind::LParen {
      return Err((
        ErrorKind::SyntaxError("Expected '(' after 'import'".to_string()),
        start_span,
      ));
    }
    self.advance();

    let path = match self.current_token.clone() {
      Token {
        kind: TokenKind::String(path),
        ..
      } => self.arena.alloc_str(path),
      _ => {
        return Err((
          ErrorKind::SyntaxError(
            "Expected a string literal for the import path".to_string(),
          ),
          start_span,
        ))
      }
    };
    self.advance();

    if self.current_token.kind != TokenKind::RParen {
      return Err((
        ErrorKind::SyntaxError("Expected ')' after import path".to_string()),
        start_span,
      ));
    }
    self.advance();

    Ok(Expression::new(
      ExpressionKind::Import { path, items: None },
      start_span,
    ))
  }
}
