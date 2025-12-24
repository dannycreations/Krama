mod base;
mod binary;
mod call;
mod collection;
mod control;
mod function;
mod import;
mod index;
mod member;
mod object;

use krama_core::{
  ErrorKind, Expression, ExpressionKind, PrecedenceKind, TokenKind,
  UpdateOperator,
};

use super::{ParseResult, Parser};

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  /// Central entry point for parsing expressions with precedence.
  pub fn parse_expression(
    &mut self,
    precedence: PrecedenceKind,
  ) -> ParseResult<'a, 'ast> {
    let mut left = self.parse_pratt()?;

    while precedence < self.current_precedence() {
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

  /// Pratt parsing prefix dispatch.
  fn parse_pratt(&mut self) -> ParseResult<'a, 'ast> {
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
      TokenKind::LBracket => self.parse_collection_expression(),
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

  /// Parses 'this' or struct construction.
  fn parse_this_expression(&mut self) -> ParseResult<'a, 'ast> {
    let span = self.current_token.span;
    self.advance();
    if self.current_token.kind == TokenKind::LBrace {
      let mut obj_parser = self.clone();
      if let Ok(expr) = obj_parser.parse_object_expression() {
        *self = obj_parser;
        if let ExpressionKind::Object { properties } = expr.kind {
          return Ok(Expression::new(
            ExpressionKind::StructConstruction { properties },
            span.merge(&expr.span),
          ));
        }
      }
      return Err(ErrorKind::SyntaxError("Invalid struct construction".into()));
    }
    Ok(Expression::new(ExpressionKind::This, span))
  }

  /// Parses a block or an object literal depending on content.
  fn parse_block_or_object_expression(&mut self) -> ParseResult<'a, 'ast> {
    let mut obj_parser = self.clone();
    if let Ok(expr) = obj_parser.parse_object_expression() {
      *self = obj_parser;
      return Ok(expr);
    }
    let block = self.arena.alloc(self.parse_block_statement()?);
    Ok(Expression::new(ExpressionKind::Block(block), block.span))
  }

  /// Parses an expression followed by a type annotation.
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

  /// Parses postfix operators (++, --, ?).
  fn parse_postfix_expression(
    &mut self,
    left: Expression<'ast>,
  ) -> ParseResult<'a, 'ast> {
    let token = self.current_token.clone();
    self.advance();
    let span = left.span.merge(&token.span);
    let kind = match token.kind {
      TokenKind::PlusPlus => ExpressionKind::Update {
        operator: UpdateOperator::Increment,
        argument: self.arena.alloc(left),
        prefix: false,
      },
      TokenKind::MinusMinus => ExpressionKind::Update {
        operator: UpdateOperator::Decrement,
        argument: self.arena.alloc(left),
        prefix: false,
      },
      TokenKind::Question => ExpressionKind::Try(self.arena.alloc(left)),
      _ => unreachable!(),
    };
    Ok(Expression::new(kind, span))
  }
}
