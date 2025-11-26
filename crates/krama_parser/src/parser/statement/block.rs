use bumpalo::collections::Vec as BumpVec;
use krama_core::ast::statement::BlockStatement;
use krama_core::error::{Error, ErrorKind};
use krama_core::span::Span;
use krama_core::token::TokenKind;

use super::Parser;

impl<'a, 'ast> Parser<'a, 'ast>
where
  'a: 'ast,
{
  pub(in crate::parser) fn parse_block_statement(
    &mut self,
  ) -> Result<BlockStatement<'ast>, Error> {
    let start_span = self.current_token.span;
    self.advance();
    let mut statements = BumpVec::new_in(self.arena);

    while self.current_token.kind != TokenKind::RBrace {
      while self.current_token.kind == TokenKind::Newline {
        self.advance();
      }

      if self.current_token.kind != TokenKind::RBrace {
        statements.push(self.parse_statement()?);
      }
    }

    if self.current_token.kind == TokenKind::Eof {
      return Err(Error {
        span: start_span,
        kind: ErrorKind::SyntaxError(
          "Unexpected end of file: missing '}'".to_string(),
        ),
      });
    }

    let end_span = self.current_token.span;
    self.advance();
    Ok(BlockStatement {
      statements,
      span: Span::new(start_span.start, end_span.end),
    })
  }
}
