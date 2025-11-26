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
  StarStar,
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
      // It is safe to cast the lifetime here because the other
      // variants do not contain any data with a lifetime.
      _ => unsafe {
        std::mem::transmute::<TokenKind<'a>, TokenKind<'static>>(self)
      },
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
