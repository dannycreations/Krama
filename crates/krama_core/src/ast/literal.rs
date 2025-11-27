use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Literal<'ast> {
  Integer(i64),
  Float(f64),
  String(&'ast str),
  Boolean(bool),
  Null,
}

impl<'ast> fmt::Display for Literal<'ast> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Literal::Integer(i) => write!(f, "{}", i),
      Literal::Float(fl) => write!(f, "{}", fl),
      Literal::String(s) => write!(f, "{}", s),
      Literal::Boolean(b) => write!(f, "{}", b),
      Literal::Null => write!(f, "null"),
    }
  }
}
