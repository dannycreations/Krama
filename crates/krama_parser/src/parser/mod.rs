pub mod expression;
mod precedence;
pub mod statement;
pub mod types;

use bumpalo::{collections::Vec as BumpVec, Bump};
use krama_core::{
  ast::{expression::Expression, Program},
  error::{Error, ErrorKind},
  span::Span,
  token::{Token, TokenKind},
};
use krama_lexer::lexer::Lexer;

use self::precedence::Precedence;

type ParseError<'a> = Result<Expression<'a>, Error>;

pub struct Parser<'a, 'ast> {
  lexer: Lexer<'a>,
  current_token: Token<'a>,
  peek_token: Token<'a>,
  arena: &'ast Bump,
}

impl<'a, 'ast> Parser<'a, 'ast>
where
  'a: 'ast,
{
  pub fn new(mut lexer: Lexer<'a>, arena: &'ast Bump) -> Self {
    let eof_pos = lexer.input_len();
    let eof_token = Token::new(TokenKind::Eof, Span::new(eof_pos, eof_pos));
    let current_token = lexer.next().unwrap_or(eof_token);
    let peek_token = lexer.next().unwrap_or(eof_token);

    Self {
      lexer,
      current_token,
      peek_token,
      arena,
    }
  }

  pub(super) fn advance(&mut self) {
    self.current_token = self.peek_token;
    self.peek_token = self.lexer.next().unwrap_or_else(|| {
      let eof_pos = self.lexer.input_len();
      Token::new(TokenKind::Eof, Span::new(eof_pos, eof_pos))
    });
  }

  pub(super) fn consume_token(
    &mut self,
    expected_kind: TokenKind,
  ) -> Result<(), Error> {
    if self.current_token.kind == expected_kind {
      self.advance();
      Ok(())
    } else {
      Err(Error {
        span: self.current_token.span,
        kind: ErrorKind::SyntaxError(format!(
          "Expected token {:?}, but got {:?}",
          expected_kind, self.current_token.kind
        )),
      })
    }
  }

  pub fn parse(&mut self) -> Result<Program<'ast>, Error> {
    let mut statements = BumpVec::new_in(self.arena);
    while self.current_token.kind != TokenKind::Eof {
      if self.current_token.kind == TokenKind::Newline {
        self.advance();
        continue;
      }
      let statement = self.parse_statement()?;
      statements.push(statement);
    }
    Ok(Program { statements })
  }

  pub(super) fn parse_identifier(&mut self) -> Result<&'a str, Error> {
    if let TokenKind::Identifier(name) = self.current_token.kind {
      self.advance();
      return Ok(name);
    }

    let kind = self.current_token.kind;
    let message = if kind.is_keyword() {
      format!("Unexpected keyword: `{}`", kind)
    } else {
      format!("Expected identifier, but got `{:?}`", kind)
    };
    Err(Error {
      span: self.current_token.span,
      kind: ErrorKind::SyntaxError(message),
    })
  }

  pub(super) fn current_precedence(&self) -> Precedence {
    Precedence::from_token(self.current_token)
  }
}
