use std::iter::Peekable;

use krama_core::{
  ErrorKind, Expression, PrecedenceKind, Span, Statement, Token, TokenKind,
};
pub use krama_core::{ErrorKindResult, ErrorResult};

use crate::Lexer;

mod expression;
mod statement;
mod types;

pub type ParseResult = ErrorKindResult<Expression>;

/// Recursive descent parser for the language.
#[derive(Clone)]
pub struct Parser<'a> {
  lexer: Peekable<Lexer<'a>>,
  current_token: Token,
}

impl<'a> Parser<'a> {
  /// Initializes a new parser with a lexer.
  pub fn new(lexer: Lexer<'a>) -> Self {
    let mut lexer = lexer.peekable();
    let current_token = lexer.next().unwrap_or_else(|| {
      let eof_span = Span::new(0, 0);
      Token::new(TokenKind::Eof, eof_span)
    });

    Self {
      lexer,
      current_token,
    }
  }

  /// Advances to the next token in the stream.
  pub fn advance(&mut self) {
    self.current_token = self.lexer.next().unwrap_or_else(|| {
      let span = &self.current_token.span;
      let eof_pos = span.end;
      Token::new(TokenKind::Eof, Span::new(eof_pos, eof_pos))
    });
  }

  /// Checks if the current token matches the expected kind and advances if it does.
  pub fn consume(
    &mut self,
    expected_kind: TokenKind,
  ) -> ErrorKindResult<Token> {
    if self.current_token.kind == expected_kind {
      let token = self.current_token.clone();
      self.advance();
      Ok(token)
    } else {
      Err(self.error(format!(
        "Expected token {}, but got {}",
        expected_kind, self.current_token.kind
      )))
    }
  }

  /// Speculatively attempts to parse using the provided closure.
  /// If the closure returns an error, the parser state is restored.
  pub fn try_parse<F, T>(&mut self, f: F) -> ErrorKindResult<T>
  where
    F: FnOnce(&mut Self) -> ErrorKindResult<T>,
  {
    let mut checkpoint = self.clone();
    f(&mut checkpoint).inspect(|_| {
      *self = checkpoint;
    })
  }

  /// Parses the entire program into an AST.
  pub fn parse(&mut self) -> ErrorResult<Vec<Statement>> {
    let mut statements = Vec::new();
    while self.current_token.kind != TokenKind::Eof {
      let statement = self
        .parse_statement()
        .map_err(|kind| kind.at(self.current_token.span))?;
      statements.push(statement);
    }
    Ok(statements)
  }

  /// Parses an identifier, handling keyword conflicts.
  pub fn parse_identifier(&mut self) -> ErrorKindResult<String> {
    match &self.current_token.kind {
      TokenKind::Identifier(name) => {
        let name = name.to_string();
        self.advance();
        Ok(name)
      }
      kind => {
        let message = if kind.is_keyword() {
          format!("Unexpected keyword `{}`, expected an identifier", kind)
        } else {
          "Expected an identifier".to_string()
        };
        Err(ErrorKind::SyntaxError(message))
      }
    }
  }

  /// Returns the precedence of the current token.
  pub fn current_precedence(&self) -> PrecedenceKind {
    PrecedenceKind::from_token(&self.current_token)
  }

  /// Helper to generate a standardized syntax error at the current token.
  #[inline(always)]
  fn error(&self, message: String) -> ErrorKind {
    ErrorKind::SyntaxError(message)
  }

  /// Helper to parse a delimited list of items.
  pub fn parse_delimited<T, F>(
    &mut self,
    start: TokenKind,
    end: TokenKind,
    separator: TokenKind,
    mut f: F,
  ) -> ErrorKindResult<Vec<T>>
  where
    F: FnMut(&mut Self) -> ErrorKindResult<T>,
  {
    self.consume(start)?;
    let mut items = Vec::new();
    if self.current_token.kind != end {
      loop {
        items.push(f(self)?);
        if self.current_token.kind == end {
          break;
        }
        self.consume(separator.clone())?;
        if self.current_token.kind == end {
          break;
        }
      }
    }
    self.consume(end)?;
    Ok(items)
  }
}
