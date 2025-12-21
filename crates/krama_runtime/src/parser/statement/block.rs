use bumpalo::collections::Vec as BumpVec;
use krama_core::{BlockStatement, ErrorKind, TokenKind};

use super::Parser;

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub(in crate::parser) fn parse_block_statement(
    &mut self,
  ) -> Result<BlockStatement<'ast>, ErrorKind> {
    let start_span = self.current_token.span;
    self.advance();
    let mut statements = BumpVec::new_in(self.arena);

    while self.current_token.kind != TokenKind::RBrace
      && self.current_token.kind != TokenKind::Eof
    {
      if self.current_token.kind != TokenKind::RBrace {
        statements.push(self.parse_statement()?);
      }
    }

    if self.current_token.kind == TokenKind::Eof {
      return Err(ErrorKind::SyntaxError(format!(
        "Unexpected end of file: missing {}",
        TokenKind::RBrace
      )));
    }

    let end_span = self.current_token.span;
    self.advance();
    Ok(BlockStatement {
      statements,
      span: start_span.merge(&end_span),
    })
  }
}
