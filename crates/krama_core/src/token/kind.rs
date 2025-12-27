use std::sync::Arc;

use logos::Logos;
use strum_macros::AsRefStr;

#[derive(Debug, Clone, PartialEq, AsRefStr, Logos)]
#[strum(serialize_all = "lowercase")]
#[logos(skip r"[ \t\f]+")]
#[logos(skip r"//[^\n]*")]
#[logos(skip r"/\*([^*]|\*+[^*/])*\*+/")]
#[logos(skip r"\n")]
pub enum TokenKind {
  #[token("const")]
  Const,
  #[token("fn")]
  Fn,
  #[token("pub")]
  Pub,
  #[token("let")]
  Let,
  #[token("struct")]
  Struct,
  #[token("this")]
  This,
  #[token("if")]
  If,
  #[token("elif")]
  Elif,
  #[token("else")]
  Else,
  #[token("match")]
  Match,
  #[token("while")]
  While,
  #[token("for")]
  For,
  #[token("in")]
  In,
  #[token("return")]
  Return,
  #[token("break")]
  Break,
  #[token("continue")]
  Continue,
  #[token("test")]
  Test,
  #[token("true")]
  True,
  #[token("false")]
  False,
  #[token("import")]
  Import,
  #[token("as")]
  As,
  #[token("null")]
  Null,
  #[token("enum")]
  Enum,
  #[token("type")]
  Type,
  #[token("i8")]
  I8,
  #[token("i16")]
  I16,
  #[token("i32")]
  I32,
  #[token("i64")]
  I64,
  #[token("i128")]
  I128,
  #[token("isize")]
  Isize,
  #[token("u8")]
  U8,
  #[token("u16")]
  U16,
  #[token("u32")]
  U32,
  #[token("u64")]
  U64,
  #[token("u128")]
  U128,
  #[token("usize")]
  Usize,
  #[token("f32")]
  F32,
  #[token("f64")]
  F64,
  #[token("bool")]
  Bool,
  #[token("str")]
  Str,

  #[token("+")]
  Plus,
  #[token("++")]
  PlusPlus,
  #[token("-")]
  Minus,
  #[token("--")]
  MinusMinus,
  #[token("*")]
  Star,
  #[token("**")]
  StarStar,
  #[token("/")]
  Slash,
  #[token("%")]
  Percent,
  #[token("=")]
  Equal,
  #[token("==")]
  EqualEqual,
  #[token("!")]
  Bang,
  #[token("!=")]
  BangEqual,
  #[token(">")]
  GreaterThan,
  #[token(">=")]
  GreaterThanEqual,
  #[token("<")]
  LessThan,
  #[token("<=")]
  LessThanEqual,
  #[token("+=")]
  PlusEqual,
  #[token("-=")]
  MinusEqual,
  #[token("*=")]
  StarEqual,
  #[token("/=")]
  SlashEqual,
  #[token("%=")]
  PercentEqual,
  #[token("&")]
  Ampersand,
  #[token("&&")]
  AmpersandAmpersand,
  #[token("|")]
  Pipe,
  #[token("||")]
  PipePipe,
  #[token("^")]
  Caret,
  #[token("~")]
  Tilde,
  #[token("<<")]
  LessLess,
  #[token(">>")]
  GreaterGreater,
  #[token("&=")]
  AmpersandEqual,
  #[token("|=")]
  PipeEqual,
  #[token("^=")]
  CaretEqual,
  #[token("<<=")]
  LessLessEqual,
  #[token(">>=")]
  GreaterGreaterEqual,

  #[token("(")]
  LParen,
  #[token(")")]
  RParen,
  #[token("{")]
  LBrace,
  #[token("}")]
  RBrace,
  #[token("[")]
  LBracket,
  #[token("]")]
  RBracket,
  #[token(",")]
  Comma,
  #[token(".")]
  Dot,
  #[token("..")]
  DotDot,
  #[token("=>")]
  Arrow,
  #[token(":")]
  Colon,
  #[token(";")]
  Semicolon,
  #[token("?")]
  Question,

  #[regex(r"[0-9][0-9_]*", |lex| Arc::from(lex.slice()), priority = 2)]
  Integer(Arc<str>),
  #[regex(r"[0-9][0-9_]*(\.[0-9][0-9_]*)?([eE][+-]?[0-9][0-9_]*)?", |lex| Arc::from(lex.slice()), priority = 1)]
  Float(Arc<str>),
  #[regex(r#""([^"\\]|\\.)*""#, |lex| Arc::from(&lex.slice()[1..lex.slice().len()-1]))]
  String(Arc<str>),
  #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| Arc::from(lex.slice()))]
  Identifier(Arc<str>),

  Unknown,
  Eof,
}

impl TokenKind {
  /// Checks if the token is a reserved keyword.
  pub fn is_keyword(&self) -> bool {
    matches!(
      self,
      Self::Const
        | Self::Fn
        | Self::Pub
        | Self::Let
        | Self::Struct
        | Self::This
        | Self::If
        | Self::Elif
        | Self::Else
        | Self::Match
        | Self::While
        | Self::For
        | Self::In
        | Self::Return
        | Self::Break
        | Self::Continue
        | Self::Test
        | Self::True
        | Self::False
        | Self::Import
        | Self::As
        | Self::Null
        | Self::Enum
        | Self::Type
        | Self::I8
        | Self::I16
        | Self::I32
        | Self::I64
        | Self::I128
        | Self::Isize
        | Self::U8
        | Self::U16
        | Self::U32
        | Self::U64
        | Self::U128
        | Self::Usize
        | Self::F32
        | Self::F64
        | Self::Bool
        | Self::Str
    )
  }
}
