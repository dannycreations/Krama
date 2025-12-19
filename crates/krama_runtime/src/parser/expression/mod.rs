mod binary;
mod call;
mod collection;
mod control;
mod function;
mod group;
mod import;
mod index;
mod literal;
mod member;
mod object;
mod unary;

use krama_core::{
  ErrorKind, Expression, ExpressionKind, Precedence, TokenKind,
};

use super::{ParseResult, Parser};

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub fn parse_expression(
    &mut self,
    precedence: Precedence,
  ) -> ParseResult<'a, 'ast> {
    let mut left = self.parse_pratt()?;

    while precedence < self.current_precedence() {
      if self.current_token.kind == TokenKind::Newline {
        break;
      }

      left = match self.current_token.kind {
        TokenKind::LParen => self.parse_call_expression(left)?,
        TokenKind::Dot => self.parse_member_expression(left)?,
        TokenKind::LBracket => self.parse_index_expression(left)?,
        TokenKind::Colon => self.parse_typed_expression(left)?,
        TokenKind::PlusPlus | TokenKind::MinusMinus | TokenKind::Question => {
          self.parse_postfix_expression(left)?
        }
        _ => self.parse_infix_expression(left)?,
      };
    }

    Ok(left)
  }

  fn parse_pratt(&mut self) -> ParseResult<'a, 'ast> {
    let token = self.current_token.clone();

    match token.kind {
      TokenKind::Identifier(_) => self.parse_identifier_expression(),
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
      TokenKind::LBracket => self.parse_collection_expression(),
      TokenKind::LBrace => {
        let mut object_parser = self.clone();
        if let Ok(expr) = object_parser.parse_object_expression() {
          *self = object_parser;
          return Ok(expr);
        }

        let block = self.arena.alloc(self.parse_block_statement()?);
        let span = block.span;
        Ok(Expression::new(ExpressionKind::Block(block), span))
      }
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

  fn parse_typed_expression(
    &mut self,
    expr: Expression<'ast>,
  ) -> ParseResult<'a, 'ast> {
    self.consume(TokenKind::Colon)?;
    let kind = self.parse_type()?;
    let span = expr.span.merge(&kind.span);
    Ok(Expression::new(
      ExpressionKind::Typed {
        expr: self.arena.alloc(expr),
        kind,
      },
      span,
    ))
  }

  fn parse_postfix_expression(
    &mut self,
    left: Expression<'ast>,
  ) -> ParseResult<'a, 'ast> {
    let token = self.current_token.clone();
    self.advance();
    match token.kind {
      TokenKind::PlusPlus => {
        let span = left.span.merge(&token.span);
        Ok(Expression::new(
          ExpressionKind::Update {
            operator: krama_core::UpdateOperator::Increment,
            argument: self.arena.alloc(left),
            prefix: false,
          },
          span,
        ))
      }
      TokenKind::MinusMinus => {
        let span = left.span.merge(&token.span);
        Ok(Expression::new(
          ExpressionKind::Update {
            operator: krama_core::UpdateOperator::Decrement,
            argument: self.arena.alloc(left),
            prefix: false,
          },
          span,
        ))
      }
      TokenKind::Question => {
        let span = left.span.merge(&token.span);
        Ok(Expression::new(
          ExpressionKind::Try(self.arena.alloc(left)),
          span,
        ))
      }
      _ => unreachable!(),
    }
  }
}
