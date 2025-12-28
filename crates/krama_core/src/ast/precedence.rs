use crate::{Token, TokenKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
  Lowest = 0,
  Assignment,     // =
  Range,          // ..
  LogicalOr,      // ||
  LogicalAnd,     // &&
  Equality,       // ==
  Comparison,     // < or >
  BitwiseOr,      // |
  BitwiseXor,     // ^
  BitwiseAnd,     // &
  Shift,          // << or >>
  Additive,       // +
  Multiplicative, // *
  Exponent,       // **
  Prefix,         // -X or !X
  Postfix,        // X++
  Call,           // myFunction(X)
  Member,         // myObject.property
  Index,          // myArray[0]
  Colon,          // :
}

impl Precedence {
  pub fn from_token(token: &Token) -> Precedence {
    match token.kind {
      TokenKind::Equal
      | TokenKind::PlusEqual
      | TokenKind::MinusEqual
      | TokenKind::StarEqual
      | TokenKind::SlashEqual
      | TokenKind::PercentEqual
      | TokenKind::AmpersandEqual
      | TokenKind::PipeEqual
      | TokenKind::CaretEqual
      | TokenKind::LessLessEqual
      | TokenKind::GreaterGreaterEqual => Precedence::Assignment,
      TokenKind::DotDot => Precedence::Range,
      TokenKind::PipePipe => Precedence::LogicalOr,
      TokenKind::AmpersandAmpersand => Precedence::LogicalAnd,
      TokenKind::EqualEqual | TokenKind::BangEqual => Precedence::Equality,
      TokenKind::LessThan
      | TokenKind::LessThanEqual
      | TokenKind::GreaterThan
      | TokenKind::GreaterThanEqual
      | TokenKind::In => Precedence::Comparison,
      TokenKind::Pipe => Precedence::BitwiseOr,
      TokenKind::Caret => Precedence::BitwiseXor,
      TokenKind::Ampersand => Precedence::BitwiseAnd,
      TokenKind::LessLess | TokenKind::GreaterGreater => Precedence::Shift,
      TokenKind::Plus | TokenKind::Minus => Precedence::Additive,
      TokenKind::Star | TokenKind::Slash | TokenKind::Percent => {
        Precedence::Multiplicative
      }
      TokenKind::StarStar => Precedence::Exponent,
      TokenKind::PlusPlus | TokenKind::MinusMinus | TokenKind::Question => {
        Precedence::Postfix
      }
      TokenKind::LParen => Precedence::Call,
      TokenKind::Dot => Precedence::Member,
      TokenKind::LBracket => Precedence::Index,
      TokenKind::Colon => Precedence::Colon,
      _ => Precedence::Lowest,
    }
  }
}
