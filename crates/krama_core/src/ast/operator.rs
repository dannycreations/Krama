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
    Some(match kind {
      TokenKind::Equal => Self::Assign,
      TokenKind::PlusEqual => Self::AddAssign,
      TokenKind::MinusEqual => Self::SubtractAssign,
      TokenKind::StarEqual => Self::MultiplyAssign,
      TokenKind::SlashEqual => Self::DivideAssign,
      TokenKind::PercentEqual => Self::ModuloAssign,
      TokenKind::AmpersandEqual => Self::BitwiseAndAssign,
      TokenKind::PipeEqual => Self::BitwiseOrAssign,
      TokenKind::CaretEqual => Self::BitwiseXorAssign,
      TokenKind::LessLessEqual => Self::LeftShiftAssign,
      TokenKind::GreaterGreaterEqual => Self::RightShiftAssign,
      _ => return None,
    })
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
    Some(match kind {
      TokenKind::Plus => Self::Add,
      TokenKind::Minus => Self::Subtract,
      TokenKind::Star => Self::Multiply,
      TokenKind::StarStar => Self::Exponent,
      TokenKind::Slash => Self::Divide,
      TokenKind::Percent => Self::Modulo,
      TokenKind::EqualEqual => Self::Equal,
      TokenKind::BangEqual => Self::NotEqual,
      TokenKind::LessThan => Self::LessThan,
      TokenKind::LessThanEqual => Self::LessThanOrEqual,
      TokenKind::GreaterThan => Self::GreaterThan,
      TokenKind::GreaterThanEqual => Self::GreaterThanOrEqual,
      TokenKind::AmpersandAmpersand => Self::LogicalAnd,
      TokenKind::PipePipe => Self::LogicalOr,
      TokenKind::Ampersand => Self::BitwiseAnd,
      TokenKind::Pipe => Self::BitwiseOr,
      TokenKind::Caret => Self::BitwiseXor,
      TokenKind::LessLess => Self::LeftShift,
      TokenKind::GreaterGreater => Self::RightShift,
      TokenKind::DotDot => Self::Range,
      TokenKind::In => Self::In,
      _ => return None,
    })
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
    Some(match kind {
      TokenKind::Bang => Self::Not,
      TokenKind::Minus => Self::Negate,
      TokenKind::Tilde => Self::BitwiseNot,
      _ => return None,
    })
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
    Some(match kind {
      TokenKind::PlusPlus => Self::Increment,
      TokenKind::MinusMinus => Self::Decrement,
      _ => return None,
    })
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
