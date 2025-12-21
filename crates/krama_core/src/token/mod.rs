use std::fmt;

use logos::Logos;
use strum_macros::AsRefStr;

use super::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
  pub kind: TokenKind<'a>,
  pub span: Span,
}

impl<'a> Token<'a> {
  pub fn new(kind: TokenKind<'a>, span: Span) -> Self {
    Self { kind, span }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, AsRefStr, Logos)]
#[strum(serialize_all = "lowercase")]
#[logos(skip r"[ \t\f]+")]
#[logos(skip r"//[^\n]*")]
#[logos(skip r"/\*([^*]|\*+[^*/])*\*+/")]
#[logos(skip r"\n")]
pub enum TokenKind<'a> {
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

  #[regex(r"[0-9][0-9_]*", |lex| lex.slice(), priority = 2)]
  Integer(&'a str),
  #[regex(r"[0-9][0-9_]*(\.[0-9][0-9_]*)?([eE][+-]?[0-9][0-9_]*)?", |lex| lex.slice(), priority = 1)]
  Float(&'a str),
  #[regex(r#""([^"\\]|\\.)*""#, |lex| &lex.slice()[1..lex.slice().len()-1])]
  String(&'a str),
  #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice())]
  Identifier(&'a str),

  Unknown,
  Eof,
}

impl<'a> TokenKind<'a> {
  pub fn is_keyword(&self) -> bool {
    matches!(
      self,
      TokenKind::Const
        | TokenKind::Fn
        | TokenKind::Pub
        | TokenKind::Let
        | TokenKind::Struct
        | TokenKind::This
        | TokenKind::If
        | TokenKind::Elif
        | TokenKind::Else
        | TokenKind::Match
        | TokenKind::While
        | TokenKind::For
        | TokenKind::In
        | TokenKind::Return
        | TokenKind::Break
        | TokenKind::Continue
        | TokenKind::Test
        | TokenKind::True
        | TokenKind::False
        | TokenKind::Import
        | TokenKind::As
        | TokenKind::Null
        | TokenKind::Enum
        | TokenKind::Type
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

impl<'a> fmt::Display for TokenKind<'a> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.is_keyword() {
      return f.write_str(self.as_ref());
    }
    let s: &dyn fmt::Display = match self {
      TokenKind::Plus => &"+",
      TokenKind::PlusPlus => &"++",
      TokenKind::Minus => &"-",
      TokenKind::MinusMinus => &"--",
      TokenKind::Star => &"*",
      TokenKind::StarStar => &"**",
      TokenKind::Slash => &"/",
      TokenKind::Percent => &"%",
      TokenKind::Equal => &"=",
      TokenKind::EqualEqual => &"==",
      TokenKind::Bang => &"!",
      TokenKind::BangEqual => &"!=",
      TokenKind::GreaterThan => &">",
      TokenKind::GreaterThanEqual => &">=",
      TokenKind::LessThan => &"<",
      TokenKind::LessThanEqual => &"<=",
      TokenKind::PlusEqual => &"+=",
      TokenKind::MinusEqual => &"-=",
      TokenKind::StarEqual => &"*=",
      TokenKind::SlashEqual => &"/=",
      TokenKind::PercentEqual => &"%=",
      TokenKind::Ampersand => &"&",
      TokenKind::AmpersandAmpersand => &"&&",
      TokenKind::Pipe => &"|",
      TokenKind::PipePipe => &"||",
      TokenKind::Caret => &"^",
      TokenKind::Tilde => &"~",
      TokenKind::LessLess => &"<<",
      TokenKind::GreaterGreater => &">>",
      TokenKind::AmpersandEqual => &"&=",
      TokenKind::PipeEqual => &"|=",
      TokenKind::CaretEqual => &"^=",
      TokenKind::LessLessEqual => &"<<=",
      TokenKind::GreaterGreaterEqual => &">>=",
      TokenKind::LParen => &"(",
      TokenKind::RParen => &")",
      TokenKind::LBrace => &"{",
      TokenKind::RBrace => &"}",
      TokenKind::LBracket => &"[",
      TokenKind::RBracket => &"]",
      TokenKind::Comma => &",",
      TokenKind::Dot => &".",
      TokenKind::DotDot => &"..",
      TokenKind::Arrow => &"=>",
      TokenKind::Colon => &":",
      TokenKind::Semicolon => &";",
      TokenKind::Question => &"?",
      TokenKind::Unknown => &"Unknown",
      TokenKind::Eof => &"Eof",
      TokenKind::Integer(s) => s,
      TokenKind::Float(s) => s,
      TokenKind::String(s) => s,
      TokenKind::Identifier(s) => s,
      _ => unreachable!(),
    };
    write!(f, "{}", s)
  }
}
