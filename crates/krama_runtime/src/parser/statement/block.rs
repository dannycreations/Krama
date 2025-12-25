use krama_core::{ErrorKind, ErrorKindResult, StatementBlock, TokenKind};

use super::Parser;

impl<'a> Parser<'a> {
  pub fn parse_block_statement(&mut self) -> ErrorKindResult<StatementBlock> {
    let start_span = self.current_token.span;
    self.advance();
    let mut statements = Vec::new();

    while self.current_token.kind != TokenKind::RBrace
      && self.current_token.kind != TokenKind::Eof
    {
      statements.push(self.parse_statement()?);
    }

    if self.current_token.kind == TokenKind::Eof {
      return Err(ErrorKind::SyntaxError(format!(
        "Unexpected end of file: missing {}",
        TokenKind::RBrace
      )));
    }

    let end_span = self.current_token.span;
    self.advance();
    Ok(StatementBlock {
      statements,
      span: start_span.merge(&end_span),
    })
  }
}
