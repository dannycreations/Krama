use std::{fmt, sync::Arc};

use logos::Logos;
use strum_macros::AsRefStr;

use super::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
  pub kind: TokenKind,
  pub span: Span,
}

impl Token {
  pub fn new(kind: TokenKind, span: Span) -> Self {
    Self { kind, span }
  }
}

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

impl fmt::Display for TokenKind {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Plus => write!(f, "+"),
      Self::PlusPlus => write!(f, "++"),
      Self::Minus => write!(f, "-"),
      Self::MinusMinus => write!(f, "--"),
      Self::Star => write!(f, "*"),
      Self::StarStar => write!(f, "**"),
      Self::Slash => write!(f, "/"),
      Self::Percent => write!(f, "%"),
      Self::Equal => write!(f, "="),
      Self::EqualEqual => write!(f, "=="),
      Self::Bang => write!(f, "!"),
      Self::BangEqual => write!(f, "!="),
      Self::GreaterThan => write!(f, ">"),
      Self::GreaterThanEqual => write!(f, ">="),
      Self::LessThan => write!(f, "<"),
      Self::LessThanEqual => write!(f, "<="),
      Self::PlusEqual => write!(f, "+="),
      Self::MinusEqual => write!(f, "-="),
      Self::StarEqual => write!(f, "*="),
      Self::SlashEqual => write!(f, "/="),
      Self::PercentEqual => write!(f, "%="),
      Self::Ampersand => write!(f, "&"),
      Self::AmpersandAmpersand => write!(f, "&&"),
      Self::Pipe => write!(f, "|"),
      Self::PipePipe => write!(f, "||"),
      Self::Caret => write!(f, "^"),
      Self::Tilde => write!(f, "~"),
      Self::LessLess => write!(f, "<<"),
      Self::GreaterGreater => write!(f, ">>"),
      Self::AmpersandEqual => write!(f, "&="),
      Self::PipeEqual => write!(f, "|="),
      Self::CaretEqual => write!(f, "^="),
      Self::LessLessEqual => write!(f, "<<="),
      Self::GreaterGreaterEqual => write!(f, ">>="),
      Self::LParen => write!(f, "("),
      Self::RParen => write!(f, ")"),
      Self::LBrace => write!(f, "{{"),
      Self::RBrace => write!(f, "}}"),
      Self::LBracket => write!(f, "["),
      Self::RBracket => write!(f, "]"),
      Self::Comma => write!(f, ","),
      Self::Dot => write!(f, "."),
      Self::DotDot => write!(f, ".."),
      Self::Arrow => write!(f, "=>"),
      Self::Colon => write!(f, ":"),
      Self::Semicolon => write!(f, ";"),
      Self::Question => write!(f, "?"),
      Self::Integer(s)
      | Self::Float(s)
      | Self::String(s)
      | Self::Identifier(s) => {
        write!(f, "{}", s)
      }
      Self::Unknown => write!(f, "Unknown"),
      Self::Eof => write!(f, "Eof"),
      // Keywords use strum's as_ref() for lowercase representation
      _ => write!(f, "{}", self.as_ref()),
    }
  }
}
