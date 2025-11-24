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
use krama_core::token::Token;
use krama_core::token::TokenKind;
use krama_lexer::lexer::Lexer;
use std::iter::Peekable;

type ParseError<'a> = Result<Expression<'a>, Error>;

pub struct Parser<'a, 'ast> {
  lexer: Peekable<Lexer<'a>>,
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
      lexer: lexer.peekable(),
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
    match self.current_token.as_ref() {
      Some(token) if token.kind == expected_kind => {
        self.advance();
        Ok(())
      }
      Some(token) => Err(Error {
        span: token.span,
        kind: ErrorKind::UnexpectedToken {
          expected: expected_kind.into_static(),
          found: token.kind.into_static(),
        },
      }),
      None => Err(Error {
        span: self.peek_token.as_ref().unwrap().span,
        kind: ErrorKind::ParserErrorOwned(format!(
          "Expected token {:?}, but got None",
          expected_kind
        )),
      }),
    }
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

  pub(super) fn current_precedence(&self) -> Precedence {
    self
      .current_token
      .as_ref()
      .map_or(Precedence::Lowest, |token| {
        if token.kind == TokenKind::Star
          && self
            .peek_token
            .as_ref()
            .is_some_and(|t| t.kind == TokenKind::Star)
        {
          Precedence::Exponent
        } else {
          Precedence::from_token(*token)
        }
      })
  }
}
