pub(super) mod binary;
pub(super) mod call;
pub(super) mod collection;
pub(super) mod control;
pub(super) mod function;
pub(super) mod group;
pub(super) mod import;
pub(super) mod index;
pub(super) mod literal;
pub(super) mod member;
pub(super) mod unary;

use krama_core::{
  ast::{
    expression::{Expression, ExpressionKind},
    precedence::Precedence,
  },
  error::{Error, ErrorKind},
  token::TokenKind,
};

use super::{ParseError, Parser};

impl<'a, 'ast> Parser<'a, 'ast> {
  pub(super) fn parse_expression(
    &mut self,
    precedence: Precedence,
  ) -> ParseError<'ast> {
    let mut left = self.parse_pratt()?;

    while precedence < self.current_precedence() {
      if self.current_token.kind == TokenKind::Newline {
        break;
      }

      left = match self.current_token.kind {
        TokenKind::LParen => self.parse_call_expression(left)?,
        TokenKind::Dot => self.parse_member_expression(left)?,
        TokenKind::LBracket => self.parse_index_expression(left)?,
        TokenKind::PlusPlus | TokenKind::MinusMinus => {
          self.parse_postfix_expression(left)?
        }
        _ => self.parse_infix_expression(left)?,
      };
    }

    Ok(left)
  }

  fn parse_pratt(&mut self) -> ParseError<'ast> {
    let token = self.current_token;

    match token.kind {
      TokenKind::Identifier(_) => self.parse_identifier_expression(),
      TokenKind::Integer(_) => self.parse_integer(),
      TokenKind::Float(_) => self.parse_float(),
      TokenKind::String(_) => self.parse_string(),
      TokenKind::True | TokenKind::False => self.parse_boolean(),
      TokenKind::Null => self.parse_null(),
      TokenKind::Bang
      | TokenKind::Minus
      | TokenKind::Tilde
      | TokenKind::Plus => self.parse_unary_expression(),
      TokenKind::PlusPlus | TokenKind::MinusMinus => {
        self.parse_prefix_update_expression()
      }
      TokenKind::LParen => self.parse_paren_expression(),
      TokenKind::LBracket => self.parse_collection_expression(),
      TokenKind::At => self.parse_import_expression(),
      TokenKind::If => self.parse_if_expression(),
      TokenKind::Match => self.parse_match_expression(),
      TokenKind::Fn => self.parse_fn_expression(),
      TokenKind::LBrace => {
        let block = self.arena.alloc(self.parse_block_statement()?);
        let span = block.span;
        Ok(Expression::new(ExpressionKind::Block(block), span))
      }
      _ => Err(Error {
        span: token.span,
        kind: ErrorKind::SyntaxError(format!(
          "Unexpected token for prefix expression: {:?}",
          token.kind
        )),
      }),
    }
  }
}
