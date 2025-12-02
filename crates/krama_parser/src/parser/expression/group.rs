use bumpalo::collections::Vec as BumpVec;
use krama_core::{
  ast::precedence::Precedence, error::ErrorKind, token::TokenKind,
};

use crate::parser::{ParseResult, Parser};

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub(super) fn parse_paren_expression(&mut self) -> ParseResult<'a, 'ast> {
    let start_span = self.current_token.span.clone();
    self.consume(TokenKind::LParen)?;

    // Handle `()` which could be an empty parameter list for an arrow function.
    if self.current_token.kind == TokenKind::RParen {
      self.consume(TokenKind::RParen)?;
      if self.current_token.kind == TokenKind::Arrow {
        // This is `() => ...`
        return self
          .build_fn_expression(start_span, BumpVec::new_in(self.arena));
      } else {
        // `()` is not a valid expression by itself.
        return Err(ErrorKind::SyntaxError(
          "Parenthesized expression cannot be empty. Use `null` for a null value."
            .to_string(),
        ));
      }
    }

    // To distinguish between a grouped expression `(a + b)` and an arrow
    // function `(a, b) => ...`, we can try to parse it as function parameters.
    // We clone the parser to backtrack if it's not a function.
    let mut fn_parser = self.clone();
    if let Ok(parameters) = fn_parser.parse_fn_parameters() {
      if fn_parser.current_token.kind == TokenKind::Arrow {
        // It is an arrow function. Commit the parser state.
        *self = fn_parser;
        return self.build_fn_expression(start_span, parameters);
      }
    }

    // If it's not an arrow function, it must be a grouped expression.
    let expression = self.parse_expression(Precedence::Lowest)?;
    self.consume(TokenKind::RParen)?;
    Ok(expression)
  }
}
