use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LiteralKind<'ast> {
  Integer(i64),
  Float(f64),
  String(&'ast str),
  Boolean(bool),
  Null,
}

impl<'ast> Display for LiteralKind<'ast> {
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
