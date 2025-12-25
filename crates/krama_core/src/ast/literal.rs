use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralKind {
  Integer(i64),
  Float(f64),
  String(String),
  Boolean(bool),
  Null,
}

impl Display for LiteralKind {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      LiteralKind::Integer(i) => write!(f, "{}", i),
      LiteralKind::Float(fl) => write!(f, "{}", fl),
      LiteralKind::String(s) => write!(f, "{}", s),
      LiteralKind::Boolean(b) => write!(f, "{}", b),
      LiteralKind::Null => write!(f, "null"),
    }
  }
}
