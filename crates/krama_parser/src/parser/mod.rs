pub mod expression;
pub mod statement;
pub mod types;

use std::iter::Peekable;

use bumpalo::{collections::Vec as BumpVec, Bump};
use krama_core::{
  ast::{expression::Expression, precedence::Precedence, Program},
  error::ErrorKind,
  span::Span,
  token::{Token, TokenKind},
};
use krama_lexer::lexer::Lexer;

type ParseError<'a, 'ast> = Result<Expression<'ast>, (ErrorKind, Span<'a>)>;

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
  pub fn new(lexer: Lexer<'a>, arena: &'ast Bump) -> Self {
    let mut lexer = lexer.peekable();
    let current_token = lexer.next().unwrap_or_else(|| {
      let eof_span = Span::new(0, 0, Some(""), None);
      Token::new(TokenKind::Eof, eof_span)
    });

    Self {
      lexer,
      current_token,
      arena,
    }
  }

  pub(super) fn advance(&mut self) {
    self.current_token = self.lexer.next().unwrap_or_else(|| {
      let span = &self.current_token.span;
      let eof_pos = span.end;
      Token::new(
        TokenKind::Eof,
        Span::new(eof_pos, eof_pos, span.source, span.file),
      )
    });
  }

  pub(super) fn consume(
    &mut self,
    expected_kind: TokenKind,
  ) -> Result<Token<'a>, (ErrorKind, Span<'a>)> {
    if self.current_token.kind == expected_kind {
      let token = self.current_token.clone();
      self.advance();
      Ok(token)
    } else {
      Err(self.expected_token_error(expected_kind))
    }
  }

  pub fn parse(&mut self) -> Result<Program<'ast>, (ErrorKind, Span<'a>)> {
    let mut statements = BumpVec::new_in(self.arena);
    while self.current_token.kind != TokenKind::Eof {
      let statement = self.parse_statement()?;
      statements.push(statement);
    }
    Ok(Program { statements })
  }

  pub(super) fn parse_identifier(
    &mut self,
  ) -> Result<&'a str, (ErrorKind, Span<'a>)> {
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
        Err((
          ErrorKind::SyntaxError(message),
          self.current_token.span.clone(),
        ))
      }
    }
  }

  pub(super) fn current_precedence(&self) -> Precedence {
    Precedence::from_token(&self.current_token)
  }

  fn expected_token_error(
    &self,
    expected_kind: TokenKind,
  ) -> (ErrorKind, Span<'a>) {
    (
      ErrorKind::SyntaxError(format!(
        "Expected token {}, but got {}",
        expected_kind, self.current_token.kind
      )),
      self.current_token.span.clone(),
    )
  }
}
