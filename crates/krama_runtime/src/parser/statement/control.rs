use krama_core::{
  ErrorKind, ForBinding, Precedence, Statement, StatementKind, TokenKind,
};

use super::Parser;

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub fn parse_while_statement(
    &mut self,
  ) -> Result<Statement<'ast>, ErrorKind> {
    let start_span = self.current_token.span;
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
    let start_span = self.current_token.span;
    self.advance();

    self.consume(TokenKind::LParen)?;

    let binding = self.parse_for_binding()?;

    self.consume(TokenKind::In)?;

    let iterable = self.parse_expression(Precedence::Lowest)?;

    self.consume(TokenKind::RParen)?;

    let body = self.parse_block_statement()?;

    Ok(Statement::new(
      StatementKind::For {
        binding,
        iterable: self.arena.alloc(iterable),
        body: self.arena.alloc(body),
      },
      start_span,
    ))
  }

  fn parse_for_binding(&mut self) -> Result<ForBinding<'ast>, ErrorKind> {
    match self.current_token.kind {
      TokenKind::Identifier(name) => {
        self.advance();
        Ok(ForBinding::Identifier(name))
      }
      TokenKind::LBracket => {
        self.advance();
        let mut elements = bumpalo::collections::Vec::new_in(self.arena);
        while self.current_token.kind != TokenKind::RBracket {
          elements.push(self.parse_for_binding()?);
          if self.current_token.kind == TokenKind::Comma {
            self.advance();
          } else if self.current_token.kind != TokenKind::RBracket {
            return Err(ErrorKind::SyntaxError(format!(
              "Expected ',' or ']', but got {}",
              self.current_token.kind
            )));
          }
        }
        self.consume(TokenKind::RBracket)?;
        Ok(ForBinding::Array(elements))
      }
      _ => Err(ErrorKind::SyntaxError(format!(
        "Expected identifier or '[', but got {}",
        self.current_token.kind
      ))),
    }
  }
}
