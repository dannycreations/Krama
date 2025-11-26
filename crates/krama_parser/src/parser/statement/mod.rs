use super::Parser;
use krama_core::ast::statement::Statement;
use krama_core::error::Error;
use krama_core::error::ErrorKind;
use krama_core::span::Span;
use krama_core::token::TokenKind;

pub(super) mod assignment;
pub(super) mod block;
pub(super) mod control;
pub(super) mod flow;
pub(super) mod function;
pub(super) mod public;
pub(super) mod test;

impl<'a, 'ast> Parser<'a, 'ast>
where
  'a: 'ast,
{
  pub(super) fn parse_statement(&mut self) -> Result<Statement<'ast>, Error> {
    let token = self.current_token.as_ref().ok_or_else(|| {
      let eof_pos = self.lexer.input_len();
      Error {
        span: Span::new(eof_pos, eof_pos),
        kind: ErrorKind::SyntaxError("Unexpected end of file".to_string()),
      }
    })?;

    let statement = match token.kind {
      TokenKind::Pub => self.parse_pub_statement(),
      TokenKind::Const => self.parse_const_statement(false, token.span),
      TokenKind::Fn => self.parse_fn_statement(false, token.span),
      TokenKind::Let => self.parse_let_statement(),
      TokenKind::Return => self.parse_return_statement(),
      TokenKind::Break => self.parse_break_statement(),
      TokenKind::Continue => self.parse_continue_statement(),
      TokenKind::Test => self.parse_test_statement(),
      TokenKind::While => self.parse_while_statement(),
      _ => self.parse_expression_statement(),
    }?;

    if self
      .current_token
      .as_ref()
      .is_some_and(|t| t.kind == TokenKind::Semicolon)
    {
      self.advance();
    }

    Ok(statement)
  }
}
