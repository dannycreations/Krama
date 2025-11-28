use std::mem;

use phf::{phf_map, Map};
use strum_macros::Display;

pub static KEYWORDS: Map<&'static str, TokenKind> = phf_map! {
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
};

#[derive(Debug, Clone, Copy, PartialEq, Display)]
pub enum TokenKind<'a> {
  // Keywords
  Const,
  Fn,
  Pub,
  Let,
  If,
  Elif,
  Else,
  Match,
  While,
  Return,
  Break,
  Continue,
  Test,
  True,
  False,
  Import,
  As,
  Null,

  // Types
  I8,
  I16,
  I32,
  I64,
  I128,
  Isize,
  U8,
  U16,
  U32,
  U64,
  U128,
  Usize,
  F32,
  F64,
  Bool,
  Str,

  // Literals
  Integer(&'a str),
  Float(&'a str),
  String(&'a str),

  // Identifier
  Identifier(&'a str),

  // Operators
  #[strum(to_string = "+")]
  Plus,
  #[strum(to_string = "++")]
  PlusPlus,
  #[strum(to_string = "-")]
  Minus,
  #[strum(to_string = "--")]
  MinusMinus,
  #[strum(to_string = "*")]
  Star,
  #[strum(to_string = "**")]
  StarStar,
  #[strum(to_string = "/")]
  Slash,
  #[strum(to_string = "%")]
  Percent,
  #[strum(to_string = "=")]
  Equal,
  #[strum(to_string = "==")]
  EqualEqual,
  #[strum(to_string = "!")]
  Bang,
  #[strum(to_string = "!=")]
  BangEqual,
  #[strum(to_string = ">")]
  GreaterThan,
  #[strum(to_string = ">=")]
  GreaterThanEqual,
  #[strum(to_string = "<")]
  LessThan,
  #[strum(to_string = "<=")]
  LessThanEqual,
  #[strum(to_string = "+=")]
  PlusEqual,
  #[strum(to_string = "-=")]
  MinusEqual,
  #[strum(to_string = "*=")]
  StarEqual,
  #[strum(to_string = "/=")]
  SlashEqual,
  #[strum(to_string = "%=")]
  PercentEqual,
  #[strum(to_string = "&")]
  Ampersand,
  #[strum(to_string = "&&")]
  AmpersandAmpersand,
  #[strum(to_string = "|")]
  Pipe,
  #[strum(to_string = "||")]
  PipePipe,
  #[strum(to_string = "^")]
  Caret,
  #[strum(to_string = "~")]
  Tilde,
  #[strum(to_string = "<<")]
  LessLess,
  #[strum(to_string = ">>")]
  GreaterGreater,
  #[strum(to_string = "&=")]
  AmpersandEqual,
  #[strum(to_string = "|=")]
  PipeEqual,
  #[strum(to_string = "^=")]
  CaretEqual,
  #[strum(to_string = "<<=")]
  LessLessEqual,
  #[strum(to_string = ">>=")]
  GreaterGreaterEqual,

  // Delimiters
  #[strum(to_string = "(")]
  LParen,
  #[strum(to_string = ")")]
  RParen,
  #[strum(to_string = "{{")]
  LBrace,
  #[strum(to_string = "}}")]
  RBrace,
  #[strum(to_string = "[")]
  LBracket,
  #[strum(to_string = "]")]
  RBracket,
  #[strum(to_string = ",")]
  Comma,
  #[strum(to_string = ".")]
  Dot,
  #[strum(to_string = "..")]
  DotDot,
  #[strum(to_string = "=>")]
  Arrow,
  #[strum(to_string = "@")]
  At,
  #[strum(to_string = ":")]
  Colon,
  #[strum(to_string = ";")]
  Semicolon,
  #[strum(to_string = "\n")]
  Newline,

  // Other
  Unknown,
  Eof,
}

impl<'a> TokenKind<'a> {
  pub fn into_static(self) -> TokenKind<'static> {
    match self {
      TokenKind::Integer(_) => TokenKind::Integer("..."),
      TokenKind::Float(_) => TokenKind::Float("..."),
      TokenKind::String(_) => TokenKind::String("..."),
      TokenKind::Identifier(_) => TokenKind::Identifier("..."),
      // It is safe to cast the lifetime here because the other
      // variants do not contain any data with a lifetime.
      _ => unsafe { mem::transmute::<TokenKind<'a>, TokenKind<'static>>(self) },
    }
  }

  pub fn is_keyword(&self) -> bool {
    matches!(
      self,
      TokenKind::Const
        | TokenKind::Fn
        | TokenKind::Pub
        | TokenKind::Let
        | TokenKind::If
        | TokenKind::Elif
        | TokenKind::Else
        | TokenKind::Match
        | TokenKind::While
        | TokenKind::Return
        | TokenKind::Break
        | TokenKind::Continue
        | TokenKind::Test
        | TokenKind::True
        | TokenKind::False
        | TokenKind::Import
        | TokenKind::As
        | TokenKind::Null
        | TokenKind::I8
        | TokenKind::I16
        | TokenKind::I32
        | TokenKind::I64
        | TokenKind::I128
        | TokenKind::Isize
        | TokenKind::U8
        | TokenKind::U16
        | TokenKind::U32
        | TokenKind::U64
        | TokenKind::U128
        | TokenKind::Usize
        | TokenKind::F32
        | TokenKind::F64
        | TokenKind::Bool
        | TokenKind::Str
    )
  }
}
