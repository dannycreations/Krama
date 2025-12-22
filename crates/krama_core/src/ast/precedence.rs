use crate::{Token, TokenKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PrecedenceKind {
  Lowest = 0,
  Assignment,  // =
  Range,       // ..
  LogicalOr,   // ||
  LogicalAnd,  // &&
  Equals,      // ==
  LessGreater, // < or >
  BitwiseOr,   // |
  BitwiseXor,  // ^
  BitwiseAnd,  // &
  Shift,       // << or >>
  Sum,         // +
  Product,     // *
  Exponent,    // **
  Prefix,      // -X or !X
  Postfix,     // X++
  Call,        // myFunction(X)
  Member,      // myObject.property
  Index,       // myArray[0]
  Colon,       // :
}

impl PrecedenceKind {
  pub fn from_token(token: &Token) -> PrecedenceKind {
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
      | TokenKind::GreaterGreaterEqual => PrecedenceKind::Assignment,
      TokenKind::DotDot => PrecedenceKind::Range,
      TokenKind::PipePipe => PrecedenceKind::LogicalOr,
      TokenKind::AmpersandAmpersand => PrecedenceKind::LogicalAnd,
      TokenKind::EqualEqual | TokenKind::BangEqual => PrecedenceKind::Equals,
      TokenKind::LessThan
      | TokenKind::LessThanEqual
      | TokenKind::GreaterThan
      | TokenKind::GreaterThanEqual
      | TokenKind::In => PrecedenceKind::LessGreater,
      TokenKind::Pipe => PrecedenceKind::BitwiseOr,
      TokenKind::Caret => PrecedenceKind::BitwiseXor,
      TokenKind::Ampersand => PrecedenceKind::BitwiseAnd,
      TokenKind::LessLess | TokenKind::GreaterGreater => PrecedenceKind::Shift,
      TokenKind::Plus | TokenKind::Minus => PrecedenceKind::Sum,
      TokenKind::Star | TokenKind::Slash | TokenKind::Percent => {
        PrecedenceKind::Product
      }
      TokenKind::StarStar => PrecedenceKind::Exponent,
      TokenKind::PlusPlus | TokenKind::MinusMinus | TokenKind::Question => {
        PrecedenceKind::Postfix
      }
      TokenKind::LParen => PrecedenceKind::Call,
      TokenKind::Dot => PrecedenceKind::Member,
      TokenKind::LBracket => PrecedenceKind::Index,
      TokenKind::Colon => PrecedenceKind::Colon,
      _ => PrecedenceKind::Lowest,
    }
  }
}
