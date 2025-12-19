mod expression;
mod statement;
mod types;

use std::iter::Peekable;

use bumpalo::{collections::Vec as BumpVec, Bump};
use krama_core::{
  Error, ErrorKind, Expression, Precedence, Program, Span, Token, TokenKind,
};

use crate::Lexer;

type ParseResult<'a, 'ast> = Result<Expression<'ast>, ErrorKind>;

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

  pub fn advance(&mut self) {
    self.current_token = self.lexer.next().unwrap_or_else(|| {
      let span = &self.current_token.span;
      let eof_pos = span.end;
      Token::new(
        TokenKind::Eof,
        Span::new(eof_pos, eof_pos, span.source, span.file),
      )
    });
  }

  pub fn consume(
    &mut self,
    expected_kind: TokenKind,
  ) -> Result<Token<'a>, ErrorKind> {
    if self.current_token.kind == expected_kind {
      let token = self.current_token.clone();
      self.advance();
      Ok(token)
    } else {
      Err(self.expected_token_error(expected_kind))
    }
  }

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

  pub fn current_precedence(&self) -> Precedence {
    Precedence::from_token(&self.current_token)
  }

  fn expected_token_error(&self, expected_kind: TokenKind) -> ErrorKind {
    ErrorKind::SyntaxError(format!(
      "Expected token {}, but got {}",
      expected_kind, self.current_token.kind
    ))
  }
}
