use bumpalo::collections::Vec as BumpVec;
use krama_core::{
  ast::{
    literal::Literal,
    types::{Type, TypeKind},
  },
  error::{Error, ErrorKind},
  token::TokenKind,
};

use super::Parser;

impl<'a, 'ast> Parser<'a, 'ast> {
  pub(super) fn parse_type(&mut self) -> Result<Type<'ast>, Error> {
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

  fn parse_tuple_type(&mut self) -> Result<Type<'ast>, Error> {
    let start_span = self.current_token.span;
    self.consume_token(TokenKind::LBracket)?;

    let mut types = BumpVec::new_in(self.arena);

    if self.current_token.kind != TokenKind::RBracket {
      loop {
        types.push(self.parse_type()?);

        if self.current_token.kind == TokenKind::RBracket {
          break;
        }
        self.consume_token(TokenKind::Comma)?;
        if self.current_token.kind == TokenKind::RBracket {
          // Allow trailing comma
          break;
        }
      }
    }

    let end_span = self.current_token.span;
    self.consume_token(TokenKind::RBracket)?;

    Ok(Type::new(
      TypeKind::Tuple(types),
      start_span.merge(&end_span),
    ))
  }

  fn parse_array_type(
    &mut self,
    element_type: Type<'ast>,
  ) -> Result<Type<'ast>, Error> {
    let span = element_type.span;
    self.consume_token(TokenKind::LBracket)?;

    let size = if self.current_token.kind == TokenKind::RBracket {
      None
    } else {
      let token = self.current_token;
      if let TokenKind::Integer(val) = token.kind {
        self.advance();
        let parsed_val: i64 =
          val.replace('_', "").parse().map_err(|_| Error {
            span: token.span,
            kind: ErrorKind::SyntaxError(format!(
              "Invalid integer literal for array size: '{}'",
              val
            )),
            file_path: None,
            source: None,
          })?;
        Some(Literal::Integer(parsed_val))
      } else {
        return Err(Error {
          span: token.span,
          kind: ErrorKind::SyntaxError(
            "Expected integer literal for array size".to_string(),
          ),
          file_path: None,
          source: None,
        });
      }
    };
    let end_span = self.current_token.span;
    self.consume_token(TokenKind::RBracket)?;

    Ok(Type::new(
      TypeKind::Array {
        element: self.arena.alloc(element_type),
        size,
      },
      span.merge(&end_span),
    ))
  }

  fn parse_base_type(&mut self) -> Result<Type<'ast>, Error> {
    let token = self.current_token;
    let span = token.span;
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
      _ => {
        return Err(Error {
          span,
          kind: ErrorKind::SyntaxError("Expected type".to_string()),
          file_path: None,
          source: None,
        })
      }
    };
    self.advance();
    Ok(Type::new(kind, span))
  }
}
