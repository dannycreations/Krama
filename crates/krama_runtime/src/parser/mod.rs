mod expression;
mod statement;
mod types;

use std::iter::Peekable;

use bumpalo::{collections::Vec as BumpVec, Bump};
use krama_core::{
  Error, ErrorKind, Expression, PrecedenceKind, Program, Span, Token, TokenKind,
};

use crate::Lexer;

type ParseResult<'a, 'ast> = Result<Expression<'ast>, ErrorKind>;

/// Recursive descent parser for the Krama language.
/// Uses a Bump arena for efficient AST allocation.
#[derive(Clone)]
pub struct Parser<'a, 'ast>
where
  'a: 'ast,
{
  lexer: Peekable<Lexer<'a>>,
  current_token: Token<'a>,
  arena: &'ast Bump,
}

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  /// Initializes a new parser with a lexer and an allocation arena.
  pub fn new(lexer: Lexer<'a>, arena: &'ast Bump) -> Self {
    let mut lexer = lexer.peekable();
    let current_token = lexer.next().unwrap_or_else(|| {
      let eof_span = Span::new(0, 0);
      Token::new(TokenKind::Eof, eof_span)
    });

    Self {
      lexer,
      current_token,
      arena,
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
  ) -> Result<Token<'a>, ErrorKind> {
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

  /// Parses the entire program into an AST.
  pub fn parse(&mut self) -> Result<Program<'ast>, Error<'a>> {
    let mut statements = BumpVec::new_in(self.arena);
    while self.current_token.kind != TokenKind::Eof {
      let statement = self
        .parse_statement()
        .map_err(|kind| Error::new(kind, self.current_token.span))?;
      statements.push(statement);
    }
    Ok(Program { statements })
  }

  /// Parses an identifier, handling keyword conflicts.
  pub fn parse_identifier(&mut self) -> Result<&'a str, ErrorKind> {
    match self.current_token.kind {
      TokenKind::Identifier(name) => {
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
}
