use krama_core::{
  ErrorKind, FunctionBody, Precedence, Span, Statement, StatementKind,
  TokenKind,
};

use super::Parser;

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub fn parse_fn_statement(
    &mut self,
    public: bool,
    start_span: Span,
  ) -> Result<Statement<'ast>, ErrorKind> {
    self.advance();
    let name = self.parse_identifier()?;
    self.consume(TokenKind::LParen)?;
    let parameters = self.parse_fn_parameters()?;
    self.consume(TokenKind::RParen)?;
    let kind = self.parse_optional_type()?;

    let body = if self.current_token.kind == TokenKind::LBrace {
      let block = self.parse_block_statement()?;
      FunctionBody::Block(self.arena.alloc(block))
    } else if self.current_token.kind == TokenKind::Arrow {
      self.advance();
      let expr = self.parse_expression(Precedence::Lowest)?;
      FunctionBody::Expression(self.arena.alloc(expr))
    } else {
      return Err(ErrorKind::SyntaxError("Expected function body".to_string()));
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
