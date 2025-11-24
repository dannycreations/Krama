#[derive(Debug, Clone, Copy, PartialEq, strum_macros::Display)]
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
  Plus,
  PlusPlus,
  Minus,
  MinusMinus,
  Star,
  Slash,
  Percent,
  Equal,
  EqualEqual,
  Bang,
  BangEqual,
  GreaterThan,
  GreaterThanEqual,
  LessThan,
  LessThanEqual,
  PlusEqual,
  MinusEqual,
  StarEqual,
  SlashEqual,
  PercentEqual,
  Ampersand,
  AmpersandAmpersand,
  Pipe,
  PipePipe,
  Caret,
  Tilde,
  LessLess,
  GreaterGreater,
  AmpersandEqual,
  PipeEqual,
  CaretEqual,
  LessLessEqual,
  GreaterGreaterEqual,

  // Delimiters
  LParen,
  RParen,
  LBrace,
  RBrace,
  LBracket,
  RBracket,
  Comma,
  Dot,
  DotDot,
  Arrow,
  At,
  Colon,
  Semicolon,
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
      _ => unsafe { std::mem::transmute::<TokenKind<'_>, TokenKind<'_>>(self) },
    }
  }
}
