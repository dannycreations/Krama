use krama_core::{
  ast::{
    precedence::Precedence,
    statement::{Statement, StatementKind},
  },
  error::ErrorKind,
  span::Span,
  token::TokenKind,
};

use crate::parser::Parser;

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub(super) fn parse_while_statement(
    &mut self,
  ) -> Result<Statement<'ast>, (ErrorKind, Span<'a>)> {
    let start_span = self.current_token.span.clone();
    self.advance();

    self.consume(TokenKind::LParen)?;

    let condition = self.parse_expression(Precedence::Lowest)?;

    self.consume(TokenKind::RParen)?;
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
