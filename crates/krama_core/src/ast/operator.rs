use crate::TokenKind;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum AssignmentOperator {
  Assign,
  AddAssign,
  SubtractAssign,
  MultiplyAssign,
  DivideAssign,
  ModuloAssign,
  BitwiseAndAssign,
  BitwiseOrAssign,
  BitwiseXorAssign,
  LeftShiftAssign,
  RightShiftAssign,
}

impl AssignmentOperator {
  /// Maps a token kind to its corresponding assignment operator.
  pub fn from_token(kind: TokenKind) -> Option<Self> {
    match kind {
      TokenKind::Equal => Some(Self::Assign),
      TokenKind::PlusEqual => Some(Self::AddAssign),
      TokenKind::MinusEqual => Some(Self::SubtractAssign),
      TokenKind::StarEqual => Some(Self::MultiplyAssign),
      TokenKind::SlashEqual => Some(Self::DivideAssign),
      TokenKind::PercentEqual => Some(Self::ModuloAssign),
      TokenKind::AmpersandEqual => Some(Self::BitwiseAndAssign),
      TokenKind::PipeEqual => Some(Self::BitwiseOrAssign),
      TokenKind::CaretEqual => Some(Self::BitwiseXorAssign),
      TokenKind::LessLessEqual => Some(Self::LeftShiftAssign),
      TokenKind::GreaterGreaterEqual => Some(Self::RightShiftAssign),
      _ => None,
    }
  }
}

/// Allows direct conversion from AssignmentOperator to BinaryOperator for compound assignments.
impl From<AssignmentOperator> for BinaryOperator {
  fn from(op: AssignmentOperator) -> Self {
    match op {
      AssignmentOperator::AddAssign => BinaryOperator::Add,
      AssignmentOperator::SubtractAssign => BinaryOperator::Subtract,
      AssignmentOperator::MultiplyAssign => BinaryOperator::Multiply,
      AssignmentOperator::DivideAssign => BinaryOperator::Divide,
      AssignmentOperator::ModuloAssign => BinaryOperator::Modulo,
      AssignmentOperator::BitwiseAndAssign => BinaryOperator::BitwiseAnd,
      AssignmentOperator::BitwiseOrAssign => BinaryOperator::BitwiseOr,
      AssignmentOperator::BitwiseXorAssign => BinaryOperator::BitwiseXor,
      AssignmentOperator::LeftShiftAssign => BinaryOperator::LeftShift,
      AssignmentOperator::RightShiftAssign => BinaryOperator::RightShift,
      AssignmentOperator::Assign => {
        unreachable!("Direct assignment has no binary equivalent")
      }
    }
  }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum BinaryOperator {
  Add,
  Subtract,
  Multiply,
  Divide,
  Modulo,
  Exponent,
  Equal,
  NotEqual,
  GreaterThan,
  GreaterThanOrEqual,
  LessThan,
  LessThanOrEqual,
  LogicalAnd,
  LogicalOr,
  BitwiseAnd,
  BitwiseOr,
  BitwiseXor,
  LeftShift,
  RightShift,
  Range,
  In,
}

impl BinaryOperator {
  /// Maps a token kind to its corresponding binary operator.
  pub fn from_token(kind: TokenKind) -> Option<Self> {
    match kind {
      TokenKind::Plus => Some(Self::Add),
      TokenKind::Minus => Some(Self::Subtract),
      TokenKind::Star => Some(Self::Multiply),
      TokenKind::StarStar => Some(Self::Exponent),
      TokenKind::Slash => Some(Self::Divide),
      TokenKind::Percent => Some(Self::Modulo),
      TokenKind::EqualEqual => Some(Self::Equal),
      TokenKind::BangEqual => Some(Self::NotEqual),
      TokenKind::LessThan => Some(Self::LessThan),
      TokenKind::LessThanEqual => Some(Self::LessThanOrEqual),
      TokenKind::GreaterThan => Some(Self::GreaterThan),
      TokenKind::GreaterThanEqual => Some(Self::GreaterThanOrEqual),
      TokenKind::AmpersandAmpersand => Some(Self::LogicalAnd),
      TokenKind::PipePipe => Some(Self::LogicalOr),
      TokenKind::Ampersand => Some(Self::BitwiseAnd),
      TokenKind::Pipe => Some(Self::BitwiseOr),
      TokenKind::Caret => Some(Self::BitwiseXor),
      TokenKind::LessLess => Some(Self::LeftShift),
      TokenKind::GreaterGreater => Some(Self::RightShift),
      TokenKind::DotDot => Some(Self::Range),
      TokenKind::In => Some(Self::In),
      _ => None,
    }
  }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum UnaryOperator {
  Not,
  Negate,
  BitwiseNot,
}

impl UnaryOperator {
  /// Maps a token kind to its corresponding unary operator.
  pub fn from_token(kind: TokenKind) -> Option<Self> {
    match kind {
      TokenKind::Bang => Some(Self::Not),
      TokenKind::Minus => Some(Self::Negate),
      TokenKind::Tilde => Some(Self::BitwiseNot),
      _ => None,
    }
  }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum UpdateOperator {
  Increment,
  Decrement,
}

impl UpdateOperator {
  /// Maps a token kind to its corresponding update operator.
  pub fn from_token(kind: TokenKind) -> Option<Self> {
    match kind {
      TokenKind::PlusPlus => Some(Self::Increment),
      TokenKind::MinusMinus => Some(Self::Decrement),
      _ => None,
    }
  }
}

/// Allows direct conversion from UpdateOperator to BinaryOperator.
impl From<UpdateOperator> for BinaryOperator {
  fn from(op: UpdateOperator) -> Self {
    match op {
      UpdateOperator::Increment => BinaryOperator::Add,
      UpdateOperator::Decrement => BinaryOperator::Subtract,
    }
  }
}
