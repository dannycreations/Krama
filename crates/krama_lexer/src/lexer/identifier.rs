use krama_core::token::{Token, TokenKind};
use once_cell::sync::Lazy;
use phf::phf_map;

use super::Lexer;

static KEYWORDS: Lazy<phf::Map<&'static str, TokenKind>> = Lazy::new(|| {
  phf_map! {
      "const" => TokenKind::Const,
      "fn" => TokenKind::Fn,
      "pub" => TokenKind::Pub,
      "let" => TokenKind::Let,
      "if" => TokenKind::If,
      "elif" => TokenKind::Elif,
      "else" => TokenKind::Else,
      "match" => TokenKind::Match,
      "return" => TokenKind::Return,
      "while" => TokenKind::While,
      "break" => TokenKind::Break,
      "continue" => TokenKind::Continue,
      "test" => TokenKind::Test,
      "true" => TokenKind::True,
      "false" => TokenKind::False,
      "import" => TokenKind::Import,
      "as" => TokenKind::As,
      "null" => TokenKind::Null,
      "i8" => TokenKind::I8,
      "i16" => TokenKind::I16,
      "i32" => TokenKind::I32,
      "i64" => TokenKind::I64,
      "i128" => TokenKind::I128,
      "isize" => TokenKind::Isize,
      "u8" => TokenKind::U8,
      "u16" => TokenKind::U16,
      "u32" => TokenKind::U32,
      "u64" => TokenKind::U64,
      "u128" => TokenKind::U128,
      "usize" => TokenKind::Usize,
      "f32" => TokenKind::F32,
      "f64" => TokenKind::F64,
      "bool" => TokenKind::Bool,
      "str" => TokenKind::Str,
  }
});

impl<'a> Lexer<'a> {
  pub(super) fn identifier(&mut self, start: usize) -> Token<'a> {
    while let Some(c) = self.peek() {
      if c.is_alphanumeric() || c == '_' {
        self.advance();
      } else {
        break;
      }
    }

    let value = &self.input_str[start..self.position];

    let kind = KEYWORDS
      .get(value)
      .cloned()
      .unwrap_or(TokenKind::Identifier(value));

    Token::new(kind, self.span(start))
  }
}
