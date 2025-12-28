use krama_core::{
  ErrorKind, ErrorKindResult, Precedence, Statement, StatementBlock,
  StatementKind, TokenKind,
};

use crate::Parser;

mod control;
mod declaration;
mod iteration;
mod test;

impl<'a> Parser<'a> {
  pub fn parse_statement(&mut self) -> ErrorKindResult<Statement> {
    let token = self.current_token.clone();

    let statement = match token.kind {
      TokenKind::Pub => self.parse_pub_statement(),
      TokenKind::Const => self.parse_const_statement(false, token.span),
      TokenKind::Fn => self.parse_fn_statement(false, token.span),
      TokenKind::Enum => self.parse_enum_statement(false, token.span),
      TokenKind::Struct => self.parse_struct_statement(false, token.span),
      TokenKind::Type => self.parse_type_statement(false, token.span),
      TokenKind::Let => self.parse_let_statement(),
      TokenKind::Return => self.parse_return_statement(),
      TokenKind::Break => self.parse_break_statement(),
      TokenKind::Continue => self.parse_continue_statement(),
      TokenKind::Test => self.parse_test_statement(),
      TokenKind::While => self.parse_while_statement(),
      TokenKind::For => self.parse_for_statement(),
      _ => {
        let expression = self.parse_expression(Precedence::Lowest)?;
        let span = expression.span;
        let statement_kind = StatementKind::Expression {
          expression: Box::new(expression),
        };
        Ok(Statement::new(statement_kind, span))
      }
    }?;

    if self.current_token.kind == TokenKind::Semicolon {
      self.advance();
    }

    Ok(statement)
  }

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
