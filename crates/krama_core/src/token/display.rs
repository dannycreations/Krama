use std::fmt::{self, Display, Formatter};

use super::kind::TokenKind;

impl Display for TokenKind {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
