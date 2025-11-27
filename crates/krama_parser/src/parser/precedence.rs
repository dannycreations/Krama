use krama_core::token::{Token, TokenKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Precedence {
  Lowest = 0,
  Assignment,  // =
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
}

impl Precedence {
  pub(crate) fn from_token(token: Token) -> Precedence {
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
      TokenKind::PipePipe => Precedence::LogicalOr,
      TokenKind::AmpersandAmpersand => Precedence::LogicalAnd,
      TokenKind::EqualEqual | TokenKind::BangEqual => Precedence::Equals,
      TokenKind::LessThan
      | TokenKind::LessThanEqual
      | TokenKind::GreaterThan
      | TokenKind::GreaterThanEqual => Precedence::LessGreater,
      TokenKind::Pipe => Precedence::BitwiseOr,
      TokenKind::Caret => Precedence::BitwiseXor,
      TokenKind::Ampersand => Precedence::BitwiseAnd,
      TokenKind::LessLess | TokenKind::GreaterGreater => Precedence::Shift,
      TokenKind::Plus | TokenKind::Minus => Precedence::Sum,
      TokenKind::Star | TokenKind::Slash | TokenKind::Percent => {
        Precedence::Product
      }
      TokenKind::StarStar => Precedence::Exponent,
      TokenKind::PlusPlus | TokenKind::MinusMinus => Precedence::Postfix,
      TokenKind::LParen => Precedence::Call,
      TokenKind::Dot => Precedence::Member,
      TokenKind::LBracket => Precedence::Index,
      _ => Precedence::Lowest,
    }
  }
}
