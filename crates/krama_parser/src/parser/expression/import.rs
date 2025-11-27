use krama_core::{
  ast::expression::{Expression, ExpressionKind},
  error::{Error, ErrorKind},
  token::{Token, TokenKind},
};

use super::{ParseError, Parser};

impl<'a, 'ast> Parser<'a, 'ast> {
  pub(super) fn parse_import_expression(&mut self) -> ParseError<'ast> {
    let start_span = self.current_token.span;
    self.advance();
    if self.current_token.kind != TokenKind::Import {
      return Err(Error {
        span: start_span,
        kind: ErrorKind::SyntaxError("Expected 'import' after '@'".to_string()),
      });
    }
    self.advance();

    if self.current_token.kind != TokenKind::LParen {
      return Err(Error {
        span: start_span,
        kind: ErrorKind::SyntaxError(
          "Expected '(' after '@import'".to_string(),
        ),
      });
    }
    self.advance();

    let path = match self.current_token {
      Token {
        kind: TokenKind::String(path),
        ..
      } => self.arena.alloc_str(path),
      _ => {
        return Err(Error {
          span: start_span,
          kind: ErrorKind::SyntaxError(
            "Expected a string literal for the import path".to_string(),
          ),
        })
      }
    };
    self.advance();

    if self.current_token.kind != TokenKind::RParen {
      return Err(Error {
        span: start_span,
        kind: ErrorKind::SyntaxError(
          "Expected ')' after import path".to_string(),
        ),
      });
    }
    self.advance();

    Ok(Expression::new(
      ExpressionKind::Import { path, items: None },
      start_span,
    ))
  }
}
