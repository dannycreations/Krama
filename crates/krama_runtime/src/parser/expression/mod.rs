use krama_core::{ErrorKind, Precedence, TokenKind};

use crate::{ParseResult, Parser};

mod call;
mod control;
mod literal;
mod primary;
mod update;

impl<'a> Parser<'a> {
  /// Central entry point for parsing expressions with precedence.
  pub fn parse_expression(&mut self, precedence: Precedence) -> ParseResult {
    let mut left = self.parse_pratt()?;

    while precedence < self.current_precedence() {
      left = match self.current_token.kind {
        TokenKind::LParen => self.parse_call_expression(left),
        TokenKind::Dot => self.parse_member_expression(left),
        TokenKind::LBracket => self.parse_index_expression(left),
        TokenKind::Colon => self.parse_typed_expression(left),
        TokenKind::PlusPlus | TokenKind::MinusMinus | TokenKind::Question => {
          self.parse_postfix_expression(left)
        }
        _ => self.parse_infix_expression(left),
      }?;
    }

    Ok(left)
  }

  /// Pratt parsing prefix dispatch.
  fn parse_pratt(&mut self) -> ParseResult {
    let token = self.current_token.clone();

    match token.kind {
      TokenKind::Identifier(_) => self.parse_identifier_expression(),
      TokenKind::This => self.parse_this_expression(),
      TokenKind::Integer(_)
      | TokenKind::Float(_)
      | TokenKind::String(_)
      | TokenKind::True
      | TokenKind::False
      | TokenKind::Null => self.parse_literal(),
      TokenKind::Bang
      | TokenKind::Minus
      | TokenKind::Tilde
      | TokenKind::Plus => self.parse_unary_expression(),
      TokenKind::PlusPlus | TokenKind::MinusMinus => {
        self.parse_prefix_update_expression()
      }
      TokenKind::LParen => self.parse_paren_expression(),
      TokenKind::LBracket => self.parse_array_expression(),
      TokenKind::LBrace => self.parse_block_or_object_expression(),
      TokenKind::Import => self.parse_import_expression(),
      TokenKind::If => self.parse_if_expression(),
      TokenKind::Match => self.parse_match_expression(),
      TokenKind::Fn => self.parse_fn_expression(),
      _ => Err(ErrorKind::SyntaxError(format!(
        "Unexpected token for prefix expression: {}",
        token.kind
      ))),
    }
  }
}
