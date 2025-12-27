use std::sync::Arc;

use krama_core::{
  AssignmentOperator, BinaryOperator, ErrorKind, Expression, ExpressionKind,
  LiteralKind, PrecedenceKind, TokenKind,
};

use crate::{ParseResult, Parser};

impl<'a> Parser<'a> {
  pub fn parse_identifier_expression(&mut self) -> ParseResult {
    let token = self.current_token.clone();
    let name = self.parse_identifier()?;
    Ok(Expression::new(
      ExpressionKind::Identifier(name.into()),
      token.span,
    ))
  }

  pub fn parse_this_expression(&mut self) -> ParseResult {
    let span = self.current_token.span;
    self.advance();
    if self.current_token.kind == TokenKind::LBrace {
      if let Ok(expr) = self.try_parse(|p| p.parse_object_expression()) {
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

  pub fn parse_member_expression(&mut self, object: Expression) -> ParseResult {
    self.advance();
    let property = self.parse_expression(PrecedenceKind::Member)?;
    let span = object.span.merge(&property.span);
    Ok(Expression::new(
      ExpressionKind::Member {
        object: Box::new(object),
        property: Box::new(property),
      },
      span,
    ))
  }

  pub fn parse_index_expression(&mut self, left: Expression) -> ParseResult {
    self.advance();
    let index = self.parse_expression(PrecedenceKind::Lowest)?;
    self.consume(TokenKind::RBracket)?;
    let span = left.span.merge(&self.current_token.span);
    Ok(Expression::new(
      ExpressionKind::Index {
        object: Box::new(left),
        index: Box::new(index),
      },
      span,
    ))
  }

  pub fn parse_typed_expression(&mut self, expr: Expression) -> ParseResult {
    self.consume(TokenKind::Colon)?;
    let kind = self.parse_type()?;
    let span = expr.span.merge(&kind.span);
    Ok(Expression::new(
      ExpressionKind::Typed {
        expr: Box::new(expr),
        kind,
      },
      span,
    ))
  }

  pub fn parse_infix_expression(&mut self, left: Expression) -> ParseResult {
    let precedence = self.current_precedence();
    let token = self.current_token.clone();

    if let Some(op) = AssignmentOperator::from_token(token.kind.clone()) {
      self.advance();
      let left_span = left.span;
      let right = self.parse_expression(precedence)?;
      return Ok(Expression::new(
        ExpressionKind::Assignment {
          left: Box::new(left),
          operator: op,
          right: Box::new(right),
        },
        token.span.merge(&left_span),
      ));
    }

    if let Some(op) = BinaryOperator::from_token(token.kind) {
      self.advance();
      let left_span = left.span;
      let right = self.parse_expression(precedence)?;
      let right_span = right.span;
      return Ok(Expression::new(
        ExpressionKind::Binary {
          left: Box::new(left),
          operator: op,
          right: Box::new(right),
        },
        left_span.merge(&right_span),
      ));
    }

    Err(ErrorKind::SyntaxError("Invalid infix operator".into()))
  }

  pub fn parse_paren_expression(&mut self) -> ParseResult {
    let start_span = self.current_token.span;
    self.consume(TokenKind::LParen)?;

    if self.current_token.kind == TokenKind::RParen {
      self.consume(TokenKind::RParen)?;
      if self.current_token.kind == TokenKind::Arrow {
        let (body, kind) = self.parse_arrow_fn_body_and_return_type()?;
        return Ok(Expression::new(
          ExpressionKind::Fn {
            parameters: Vec::new(),
            body,
            kind,
          },
          start_span,
        ));
      }
      return Err(ErrorKind::SyntaxError("Empty parens".into()));
    }

    let mut fn_parser = self.clone();
    if let Ok(parameters) = fn_parser.parse_fn_parameters() {
      if fn_parser.current_token.kind == TokenKind::RParen {
        fn_parser.consume(TokenKind::RParen)?;
        if matches!(
          fn_parser.current_token.kind,
          TokenKind::Arrow | TokenKind::Colon
        ) {
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

    let expr = self.parse_expression(PrecedenceKind::Lowest)?;
    self.consume(TokenKind::RParen)?;
    Ok(expr)
  }

  pub fn parse_collection_expression(&mut self) -> ParseResult {
    let start_span = self.current_token.span;
    let elements = self.parse_delimited(
      TokenKind::LBracket,
      TokenKind::RBracket,
      TokenKind::Comma,
      |p| p.parse_expression(PrecedenceKind::Lowest),
    )?;
    let span = start_span.merge(&self.current_token.span);
    Ok(Expression::new(
      ExpressionKind::Collection { elements },
      span,
    ))
  }

  pub fn parse_object_expression(&mut self) -> ParseResult {
    let start_span = self.consume(TokenKind::LBrace)?.span;
    let mut properties = Vec::new();
    if self.current_token.kind == TokenKind::RBrace {
      let end_span = self.current_token.span;
      self.advance();
      return Ok(Expression::new(
        ExpressionKind::Object { properties },
        start_span.merge(&end_span),
      ));
    }
    loop {
      let (key, value) = match self.current_token.kind {
        TokenKind::Identifier(_) => {
          let key_span = self.current_token.span;
          let key_ident: Arc<str> = self.parse_identifier()?.into();
          let key_expr = Expression::new(
            ExpressionKind::Literal(LiteralKind::String(key_ident.clone())),
            key_span,
          );
          if self.current_token.kind == TokenKind::Colon {
            self.advance();
            let value = self.parse_expression(PrecedenceKind::Lowest)?;
            (key_expr, value)
          } else {
            let value =
              Expression::new(ExpressionKind::Identifier(key_ident), key_span);
            (key_expr, value)
          }
        }
        TokenKind::String(_) => {
          let key = self.parse_literal()?;
          self.consume(TokenKind::Colon)?;
          let value = self.parse_expression(PrecedenceKind::Lowest)?;
          (key, value)
        }
        _ => {
          return Err(ErrorKind::SyntaxError(
            "Expected string or identifier for object key".to_string(),
          ));
        }
      };
      properties.push((key, value));
      if self.current_token.kind == TokenKind::RBrace {
        break;
      }
      self.consume(TokenKind::Comma)?;
      if self.current_token.kind == TokenKind::RBrace {
        break;
      }
    }
    let end_span = self.consume(TokenKind::RBrace)?.span;
    Ok(Expression::new(
      ExpressionKind::Object { properties },
      start_span.merge(&end_span),
    ))
  }

  pub fn parse_block_or_object_expression(&mut self) -> ParseResult {
    if let Ok(expr) = self.try_parse(|p| p.parse_object_expression()) {
      return Ok(expr);
    }
    let block = Box::new(self.parse_block_statement()?);
    let span = block.span;
    Ok(Expression::new(ExpressionKind::Block(block), span))
  }
}
