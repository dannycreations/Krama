use bumpalo::collections::Vec as BumpVec;
use krama_core::{
  ast::{
    literal::Literal,
    types::{Type, TypeKind},
  },
  error::ErrorKind,
  token::TokenKind,
};

use crate::parser::Parser;

impl<'a, 'ast> Parser<'a, 'ast>
where
  'ast: 'a,
{
  pub(super) fn parse_type(&mut self) -> Result<Type<'ast>, ErrorKind> {
    let mut kind = if self.current_token.kind == TokenKind::LBracket {
      self.parse_tuple_type()?
    } else {
      self.parse_base_type()?
    };

    while self.current_token.kind == TokenKind::LBracket {
      kind = self.parse_array_type(kind)?;
    }

    Ok(kind)
  }

  pub(super) fn parse_optional_type(
    &mut self,
  ) -> Result<Option<Type<'ast>>, ErrorKind> {
    if self.current_token.kind == TokenKind::Colon {
      self.advance();
      self.parse_type().map(Some)
    } else {
      Ok(None)
    }
  }

  fn parse_tuple_type(&mut self) -> Result<Type<'ast>, ErrorKind> {
    let start_span = self.current_token.span.clone();
    self.consume(TokenKind::LBracket)?;

    let mut types = BumpVec::new_in(self.arena);

    if self.current_token.kind != TokenKind::RBracket {
      loop {
        types.push(self.parse_type()?);

        if self.current_token.kind == TokenKind::RBracket {
          break;
        }
        self.consume(TokenKind::Comma)?;
        if self.current_token.kind == TokenKind::RBracket {
          // Allow trailing comma
          break;
        }
      }
    }

    let end_span = self.consume(TokenKind::RBracket)?.span;

    Ok(Type::new(
      TypeKind::Tuple(types),
      start_span.merge(&end_span),
    ))
  }

  fn parse_array_type(
    &mut self,
    element_type: Type<'ast>,
  ) -> Result<Type<'ast>, ErrorKind> {
    let span = element_type.span.clone();
    self.consume(TokenKind::LBracket)?;

    let size = if self.current_token.kind == TokenKind::RBracket {
      None
    } else if let TokenKind::Integer(val) = self.current_token.kind {
      self.advance();
      let parsed_val: i64 = val.replace('_', "").parse().map_err(|_| {
        ErrorKind::SyntaxError(format!(
          "Invalid integer literal for array size: '{}'",
          val
        ))
      })?;
      Some(Literal::Integer(parsed_val))
    } else {
      return Err(ErrorKind::SyntaxError(
        "Expected integer literal for array size".to_string(),
      ));
    };
    let end_span = self.consume(TokenKind::RBracket)?.span;

    Ok(Type::new(
      TypeKind::Array {
        element: self.arena.alloc(element_type),
        size,
      },
      span.merge(&end_span),
    ))
  }

  fn parse_base_type(&mut self) -> Result<Type<'ast>, ErrorKind> {
    let token = self.current_token.clone();
    let span = token.span.clone();
    let kind = match token.kind {
      TokenKind::I8 => TypeKind::I8,
      TokenKind::I16 => TypeKind::I16,
      TokenKind::I32 => TypeKind::I32,
      TokenKind::I64 => TypeKind::I64,
      TokenKind::I128 => TypeKind::I128,
      TokenKind::Isize => TypeKind::Isize,
      TokenKind::U8 => TypeKind::U8,
      TokenKind::U16 => TypeKind::U16,
      TokenKind::U32 => TypeKind::U32,
      TokenKind::U64 => TypeKind::U64,
      TokenKind::U128 => TypeKind::U128,
      TokenKind::Usize => TypeKind::Usize,
      TokenKind::F32 => TypeKind::F32,
      TokenKind::F64 => TypeKind::F64,
      TokenKind::Bool => TypeKind::Bool,
      TokenKind::Str => TypeKind::Str,
      TokenKind::Identifier(ident) => TypeKind::Identifier(ident),
      _ => return Err(ErrorKind::SyntaxError("Expected type".to_string())),
    };
    self.advance();
    Ok(Type::new(kind, span))
  }
}
