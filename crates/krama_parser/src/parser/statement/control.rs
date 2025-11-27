use krama_core::{
  ast::statement::{Statement, StatementKind},
  error::Error,
  token::TokenKind,
};

use super::{super::precedence::Precedence, Parser};

impl<'a, 'ast> Parser<'a, 'ast> {
  pub(super) fn parse_while_statement(
    &mut self,
  ) -> Result<Statement<'ast>, Error> {
    let start_span = self.current_token.span;
    self.advance();

    self.consume_token(TokenKind::LParen)?;

    let condition = self.parse_expression(Precedence::Lowest)?;

    self.consume_token(TokenKind::RParen)?;
    let body = self.parse_block_statement()?;

    Ok(Statement::new(
      StatementKind::While {
        condition: self.arena.alloc(condition),
        body: self.arena.alloc(body),
      },
      start_span,
    ))
  }
}
