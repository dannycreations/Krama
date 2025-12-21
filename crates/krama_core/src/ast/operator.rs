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

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum UnaryOperator {
  Not,
  Negate,
  BitwiseNot,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum UpdateOperator {
  Increment,
  Decrement,
}
