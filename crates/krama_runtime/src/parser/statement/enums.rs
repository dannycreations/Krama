use krama_core::{
  EnumVariant, ErrorKind, Span, Statement, StatementKind, TokenKind,
};

use super::Parser;

impl<'a> Parser<'a> {
  pub fn parse_enum_statement(
    &mut self,
    public: bool,
    start_span: Span,
  ) -> Result<Statement, ErrorKind> {
    self.consume(TokenKind::Enum)?;

    let name = self.parse_identifier()?;

    self.consume(TokenKind::LBrace)?;

    let mut variants = Vec::new();

    while self.current_token.kind != TokenKind::RBrace {
      let variant_span = self.current_token.span;
      let variant_name = self.parse_identifier()?;

      let fields = if self.current_token.kind == TokenKind::LParen {
        self.advance();
        let mut fields = Vec::new();
        if self.current_token.kind != TokenKind::RParen {
          loop {
            fields.push(self.parse_type()?);
            if self.current_token.kind == TokenKind::RParen {
              break;
            }
            self.consume(TokenKind::Comma)?;
          }
        }
        self.consume(TokenKind::RParen)?;
        Some(fields)
      } else {
        None
      };

      let end_variant_span = self.current_token.span;
      variants.push(EnumVariant {
        name: variant_name,
        fields,
        span: variant_span.merge(&end_variant_span),
      });

      if self.current_token.kind == TokenKind::Comma {
        self.advance();
      } else if self.current_token.kind != TokenKind::RBrace {
        // Optional comma
      }
    }

    let end_span = self.consume(TokenKind::RBrace)?.span;

    Ok(Statement::new(
      StatementKind::Enum {
        public,
        name,
        variants,
      },
      start_span.merge(&end_span),
    ))
  }
}
