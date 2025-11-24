#[derive(Debug, Clone, PartialEq, Copy)]
pub enum BinaryOperator {
  Add,
  Subtract,
  Multiply,
  Divide,
  Modulo,
  Exponent,
  Assign,
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
