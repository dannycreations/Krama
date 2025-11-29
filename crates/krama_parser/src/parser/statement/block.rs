use bumpalo::collections::Vec as BumpVec;
use krama_core::{
  ast::statement::BlockStatement, error::ErrorKind, span::Span,
  token::TokenKind,
};

use crate::parser::Parser;

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub(in crate::parser) fn parse_block_statement(
    &mut self,
  ) -> Result<BlockStatement<'ast>, (ErrorKind, Span<'a>)> {
    let start_span = self.current_token.span.clone();
    self.advance();
    let mut statements = BumpVec::new_in(self.arena);

    while self.current_token.kind != TokenKind::RBrace
      && self.current_token.kind != TokenKind::Eof
    {
      while self.current_token.kind == TokenKind::Newline {
        self.advance();
      }

      if self.current_token.kind != TokenKind::RBrace {
        statements.push(self.parse_statement()?);
      }
    }

    if self.current_token.kind == TokenKind::Eof {
      return Err((
        ErrorKind::SyntaxError(format!(
          "Unexpected end of file: missing {}",
          TokenKind::RBrace
        )),
        start_span,
      ));
    }

    let end_span = self.current_token.span.clone();
    self.advance();
    Ok(BlockStatement {
      statements,
      span: start_span.merge(&end_span),
    })
  }
}
