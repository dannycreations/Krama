use krama_core::{ErrorKind, Precedence, Statement, StatementKind, TokenKind};

use super::Parser;

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub fn parse_while_statement(
    &mut self,
  ) -> Result<Statement<'ast>, ErrorKind> {
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

  pub fn parse_for_statement(&mut self) -> Result<Statement<'ast>, ErrorKind> {
    let start_span = self.current_token.span.clone();
    self.advance();

    self.consume(TokenKind::LParen)?;

    let name = self.parse_identifier()?;

    self.consume(TokenKind::In)?;

    let iterable = self.parse_expression(Precedence::Lowest)?;

    self.consume(TokenKind::RParen)?;

    let body = self.parse_block_statement()?;

    Ok(Statement::new(
      StatementKind::For {
        name,
        iterable: self.arena.alloc(iterable),
        body: self.arena.alloc(body),
      },
      start_span,
    ))
  }
}
