use krama_core::{
  ErrorKind, ForBinding, PrecedenceKind, Statement, StatementKind, TokenKind,
};

use super::Parser;

impl<'a> Parser<'a> {
  pub fn parse_while_statement(&mut self) -> Result<Statement, ErrorKind> {
    let start_span = self.current_token.span;
    self.advance();

    self.consume(TokenKind::LParen)?;

    let condition = self.parse_expression(PrecedenceKind::Lowest)?;

    self.consume(TokenKind::RParen)?;
    let body = self.parse_block_statement()?;

    Ok(Statement::new(
      StatementKind::While {
        condition: Box::new(condition),
        body: Box::new(body),
      },
      start_span,
    ))
  }

  pub fn parse_for_statement(&mut self) -> Result<Statement, ErrorKind> {
    let start_span = self.current_token.span;
    self.advance();

    self.consume(TokenKind::LParen)?;

    let binding = self.parse_for_binding()?;

    self.consume(TokenKind::In)?;

    let iterable = self.parse_expression(PrecedenceKind::Lowest)?;

    self.consume(TokenKind::RParen)?;

    let body = self.parse_block_statement()?;

    Ok(Statement::new(
      StatementKind::For {
        binding,
        iterable: Box::new(iterable),
        body: Box::new(body),
      },
      start_span,
    ))
  }

  fn parse_for_binding(&mut self) -> Result<ForBinding, ErrorKind> {
    match &self.current_token.kind {
      TokenKind::Identifier(name) => {
        let name = name.to_string();
        self.advance();
        Ok(ForBinding::Identifier(name))
      }
      TokenKind::LBracket => {
        self.advance();
        let mut elements = Vec::new();
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
