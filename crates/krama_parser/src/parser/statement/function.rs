use krama_core::{
  ast::{
    expression::FunctionBody,
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
  pub(super) fn parse_fn_statement(
    &mut self,
    public: bool,
    start_span: Span<'a>,
  ) -> Result<Statement<'ast>, (ErrorKind, Span<'a>)> {
    self.advance();
    let name = if let TokenKind::Identifier(name) = self.current_token.kind {
      self.arena.alloc_str(name)
    } else {
      return Err((
        ErrorKind::SyntaxError("Expected function name after 'fn'".to_string()),
        start_span,
      ));
    };
    self.advance();
    self.consume(TokenKind::LParen)?;
    let parameters = self.parse_fn_parameters()?;
    let kind = if self.current_token.kind == TokenKind::Colon {
      self.advance();
      Some(self.parse_type()?)
    } else {
      None
    };

    let body = if self.current_token.kind == TokenKind::LBrace {
      let block = self.parse_block_statement()?;
      FunctionBody::Block(self.arena.alloc(block))
    } else if self.current_token.kind == TokenKind::Equal {
      self.advance();
      let expr = self.parse_expression(Precedence::Lowest)?;
      FunctionBody::Expression(self.arena.alloc(expr))
    } else {
      return Err((
        ErrorKind::SyntaxError("Expected function body".to_string()),
        self.current_token.span.clone(),
      ));
    };

    Ok(Statement::new(
      StatementKind::Fn {
        public,
        name,
        parameters,
        body,
        kind,
      },
      start_span,
    ))
  }
}
