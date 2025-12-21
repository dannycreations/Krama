use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Literal<'ast> {
  Integer(i64),
  Float(f64),
  String(&'ast str),
  Boolean(bool),
  Null,
}

impl<'ast> Display for Literal<'ast> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match self {
      Literal::Integer(i) => write!(f, "{}", i),
      Literal::Float(fl) => write!(f, "{}", fl),
      Literal::String(s) => write!(f, "{}", s),
      Literal::Boolean(b) => write!(f, "{}", b),
      Literal::Null => write!(f, "null"),
    }
  }
}
