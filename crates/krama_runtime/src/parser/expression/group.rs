use bumpalo::collections::Vec as BumpVec;
use krama_core::{
  ErrorKind, Expression, ExpressionKind, Precedence, TokenKind,
};

use super::{ParseResult, Parser};

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub fn parse_paren_expression(&mut self) -> ParseResult<'a, 'ast> {
    let start_span = self.current_token.span;
    self.consume(TokenKind::LParen)?;

    if self.current_token.kind == TokenKind::RParen {
      self.consume(TokenKind::RParen)?;
      if self.current_token.kind == TokenKind::Arrow {
        let (body, kind) = self.parse_arrow_fn_body_and_return_type()?;
        return Ok(Expression::new(
          ExpressionKind::Fn {
            parameters: BumpVec::new_in(self.arena),
            body,
            kind,
          },
          start_span,
        ));
      } else {
        return Err(ErrorKind::SyntaxError(
          "Parenthesized expression cannot be empty. Use `null` for a null value."
            .to_string(),
        ));
      }
    }

    let mut fn_parser = self.clone();
    if let Ok(parameters) = fn_parser.parse_fn_parameters() {
      if fn_parser.current_token.kind == TokenKind::RParen {
        fn_parser.consume(TokenKind::RParen)?;
        if fn_parser.current_token.kind == TokenKind::Arrow {
          *self = fn_parser;
          let (body, kind) = self.parse_arrow_fn_body_and_return_type()?;
          return Ok(Expression::new(
            ExpressionKind::Fn {
              parameters,
              body,
              kind,
            },
            start_span,
          ));
        }
      }
    }

    let expression = self.parse_expression(Precedence::Lowest)?;
    self.consume(TokenKind::RParen)?;
    Ok(expression)
  }
}
