use bumpalo::collections::Vec as BumpVec;
use krama_core::{
  ast::expression::{
    Expression, ExpressionKind, FunctionBody, MatchArm, MatchPattern,
  },
  error::{Error, ErrorKind},
  token::TokenKind,
};

use super::{ParseError, Parser, Precedence};

impl<'a, 'ast> Parser<'a, 'ast> {
  pub(super) fn parse_if_expression(&mut self) -> ParseError<'ast> {
    let start_span = self.current_token.span;
    self.advance();

    if self.current_token.kind != TokenKind::LParen {
      return Err(Error {
        span: self.current_token.span,
        kind: ErrorKind::SyntaxError(
          "Expected '(' after 'if' or 'elif'".to_string(),
        ),
      });
    }
    self.advance();

    let condition = self.parse_expression(Precedence::Lowest)?;

    if self.current_token.kind != TokenKind::RParen {
      return Err(Error {
        span: self.current_token.span,
        kind: ErrorKind::SyntaxError(
          "Expected ')' after if condition'".to_string(),
        ),
      });
    }
    self.advance();

    let then_branch = self.arena.alloc(self.parse_block_statement()?);
    let then_span = then_branch.span;

    let else_branch = if self.current_token.kind == TokenKind::Else {
      self.advance();
      let else_block = self.arena.alloc(self.parse_block_statement()?);
      let else_span = else_block.span;
      Some(self.arena.alloc(Expression::new(
        ExpressionKind::Block(else_block),
        else_span,
      )))
    } else if self.current_token.kind == TokenKind::Elif {
      Some(self.arena.alloc(self.parse_if_expression()?))
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
        else_branch: else_branch.map(|e| &*e),
      },
      start_span,
    ))
  }

  pub(super) fn parse_match_expression(&mut self) -> ParseError<'ast> {
    let start_span = self.current_token.span;
    self.advance();

    if self.current_token.kind != TokenKind::LParen {
      return Err(Error {
        span: start_span,
        kind: ErrorKind::SyntaxError("Expected '(' after 'match'".to_string()),
      });
    }
    self.advance();

    let subject = self.parse_expression(Precedence::Lowest)?;

    if self.current_token.kind != TokenKind::RParen {
      return Err(Error {
        span: start_span,
        kind: ErrorKind::SyntaxError(
          "Expected ')' after match subject'".to_string(),
        ),
      });
    }
    self.advance();

    if self.current_token.kind != TokenKind::LBrace {
      return Err(Error {
        span: start_span,
        kind: ErrorKind::SyntaxError("Expected '{' for match arms".to_string()),
      });
    }
    self.advance();

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
      return Err(Error {
        span: start_span,
        kind: ErrorKind::SyntaxError(
          "Unexpected end of file: missing '}'".to_string(),
        ),
      });
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

  fn parse_match_arm(&mut self) -> Result<MatchArm<'ast>, Error> {
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
      return Err(Error {
        span: self.current_token.span,
        kind: ErrorKind::SyntaxError(
          "Expected '=>' or '{' for match arm body".to_string(),
        ),
      });
    };

    if self.current_token.kind == TokenKind::Comma {
      self.advance();
    }
    while self.current_token.kind == TokenKind::Newline {
      self.advance();
    }

    Ok(MatchArm { patterns, body })
  }

  fn parse_match_pattern(&mut self) -> Result<MatchPattern<'ast>, Error> {
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
