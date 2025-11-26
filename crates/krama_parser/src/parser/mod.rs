pub mod expression;
mod precedence;
pub mod statement;
pub mod types;

use self::precedence::Precedence;
use bumpalo::collections::Vec as BumpVec;
use bumpalo::Bump;
use krama_core::ast::expression::Expression;
use krama_core::ast::Program;
use krama_core::error::Error;
use krama_core::error::ErrorKind;
use krama_core::span::Span;
use krama_core::token::Token;
use krama_core::token::TokenKind;
use krama_lexer::lexer::Lexer;

type ParseError<'a> = Result<Expression<'a>, Error>;

pub struct Parser<'a, 'ast> {
  lexer: Lexer<'a>,
  current_token: Option<Token<'a>>,
  peek_token: Option<Token<'a>>,
  arena: &'ast Bump,
}

impl<'a, 'ast> Parser<'a, 'ast>
where
  'a: 'ast,
{
  pub fn new(lexer: Lexer<'a>, arena: &'ast Bump) -> Self {
    let mut parser = Self {
      lexer,
      current_token: None,
      peek_token: None,
      arena,
    };
    parser.advance();
    parser.advance();
    parser
  }

  pub(super) fn advance(&mut self) {
    self.current_token = self.peek_token.take();
    self.peek_token = self.lexer.next();
  }

  pub(super) fn consume_token(
    &mut self,
    expected_kind: TokenKind,
  ) -> Result<(), Error> {
    if let Some(token) = self.current_token.as_ref() {
      if token.kind == expected_kind {
        self.advance();
        return Ok(());
      }
      return Err(Error {
        span: token.span,
        kind: ErrorKind::SyntaxError(format!(
          "Expected token {:?}, but got {:?}",
          expected_kind, token.kind
        )),
      });
    }

    let eof_pos = self.lexer.input_len();
    Err(Error {
      span: Span::new(eof_pos, eof_pos),
      kind: ErrorKind::SyntaxError(format!(
        "Expected token {:?}, but found end of file.",
        expected_kind,
      )),
    })
  }

  pub fn parse(&mut self) -> Result<Program<'ast>, Error> {
    let mut statements = BumpVec::new_in(self.arena);
    while self
      .current_token
      .as_ref()
      .is_some_and(|t| t.kind != TokenKind::Eof)
    {
      if self
        .current_token
        .as_ref()
        .is_some_and(|t| t.kind == TokenKind::Newline)
      {
        self.advance();
        continue;
      }
      let statement = self.parse_statement()?;
      statements.push(statement);
    }
    Ok(Program { statements })
  }

  pub(super) fn parse_identifier(&mut self) -> Result<&'a str, Error> {
    let token = self.current_token.as_ref().ok_or_else(|| {
      let eof_pos = self.lexer.input_len();
      Error {
        span: Span::new(eof_pos, eof_pos),
        kind: ErrorKind::SyntaxError(
          "Expected identifier, found nothing".to_string(),
        ),
      }
    })?;

    match token.kind {
      TokenKind::Identifier(name) => {
        self.advance();
        Ok(name)
      }
      kind if kind.is_keyword() => Err(Error {
        span: token.span,
        kind: ErrorKind::SyntaxError(format!("Unexpected keyword: `{}`", kind)),
      }),
      found => Err(Error {
        span: token.span,
        kind: ErrorKind::SyntaxError(format!(
          "Expected identifier, but got `{:?}`",
          found
        )),
      }),
    }
  }

  pub(super) fn current_precedence(&self) -> Precedence {
    self
      .current_token
      .as_ref()
      .map_or(Precedence::Lowest, |token| Precedence::from_token(*token))
  }
}
