use bumpalo::collections::Vec as BumpVec;
use krama_core::{
  ast::{
    expression::{
      Expression, ExpressionKind, FunctionBody, MatchArm, MatchPattern,
    },
    precedence::Precedence,
  },
  error::ErrorKind,
  span::Span,
  token::TokenKind,
};

use crate::parser::{ParseResult, Parser};

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub(super) fn parse_if_expression(&mut self) -> ParseResult<'a, 'ast> {
    let start_span = self.current_token.span.clone();
    self.advance();

    self.consume(TokenKind::LParen)?;

    let condition = self.parse_expression(Precedence::Lowest)?;

    self.consume(TokenKind::RParen)?;

    let then_branch = self.arena.alloc(self.parse_block_statement()?);
    let then_span = then_branch.span.clone();

    let else_branch = if self.current_token.kind == TokenKind::Else {
      self.advance();
      let else_block = self.arena.alloc(self.parse_block_statement()?);
      let else_span = else_block.span.clone();
      Some(&*self.arena.alloc(Expression::new(
        ExpressionKind::Block(else_block),
        else_span,
      )))
    } else if self.current_token.kind == TokenKind::Elif {
      Some(&*self.arena.alloc(self.parse_if_expression()?))
    } else {
      None
    };

    Ok(Expression::new(
      ExpressionKind::If {
        condition: self.arena.alloc(condition),
        then_branch: self.arena.alloc(Expression::new(
          ExpressionKind::Block(then_branch),
          then_span,
        )),
        else_branch,
      },
      start_span,
    ))
  }

  pub(super) fn parse_match_expression(&mut self) -> ParseResult<'a, 'ast> {
    let start_span = self.current_token.span.clone();
    self.advance();

    self.consume(TokenKind::LParen)?;

    let subject = self.parse_expression(Precedence::Lowest)?;

    self.consume(TokenKind::RParen)?;
    self.consume(TokenKind::LBrace)?;

    let mut arms = BumpVec::new_in(self.arena);
    while self.current_token.kind != TokenKind::RBrace {
      while self.current_token.kind == TokenKind::Newline {
        self.advance();
      }

      if self.current_token.kind != TokenKind::RBrace {
        arms.push(self.parse_match_arm()?);
      }
    }

    if self.current_token.kind == TokenKind::Eof {
      return Err((
        ErrorKind::SyntaxError(format!(
          "Unexpected end of file: missing {}",
          TokenKind::RBrace
        )),
        start_span,
      ));
    }

    self.advance();

    Ok(Expression::new(
      ExpressionKind::Match {
        subject: self.arena.alloc(subject),
        arms,
      },
      start_span,
    ))
  }

  fn parse_match_arm(
    &mut self,
  ) -> Result<MatchArm<'ast>, (ErrorKind, Span<'a>)> {
    let mut patterns = BumpVec::new_in(self.arena);
    patterns.push(self.parse_match_pattern()?);

    while self.current_token.kind == TokenKind::Comma {
      self.advance();

      if self.current_token.kind == TokenKind::Arrow
        || self.current_token.kind == TokenKind::LBrace
      {
        break;
      }

      while self.current_token.kind == TokenKind::Newline {
        self.advance();
      }

      patterns.push(self.parse_match_pattern()?);
    }

    let body = if self.current_token.kind == TokenKind::Arrow {
      self.advance();
      let expr = self.parse_expression(Precedence::Lowest)?;
      FunctionBody::Expression(self.arena.alloc(expr))
    } else if self.current_token.kind == TokenKind::LBrace {
      let block = self.arena.alloc(self.parse_block_statement()?);
      FunctionBody::Block(block)
    } else {
      return Err((
        ErrorKind::SyntaxError(format!(
          "Expected {} or {} for match arm body",
          TokenKind::Arrow,
          TokenKind::LBrace
        )),
        self.current_token.span.clone(),
      ));
    };

    if self.current_token.kind == TokenKind::Comma {
      self.advance();
    }
    while self.current_token.kind == TokenKind::Newline {
      self.advance();
    }

    Ok(MatchArm { patterns, body })
  }

  fn parse_match_pattern(
    &mut self,
  ) -> Result<MatchPattern<'ast>, (ErrorKind, Span<'a>)> {
    if self.current_token.kind == TokenKind::Else {
      self.advance();
      return Ok(MatchPattern::Else);
    }

    let left = self.parse_expression(Precedence::LessGreater)?;

    if self.current_token.kind == TokenKind::DotDot {
      self.advance();
      let right = self.parse_expression(Precedence::LessGreater)?;
      Ok(MatchPattern::Range(left, right))
    } else {
      Ok(MatchPattern::Expression(left))
    }
  }
}
