use std::{
  fmt::{Display, Formatter, Result as FmtResult},
  sync::Arc,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
  Integer(i64),
  Float(f64),
  String(Arc<str>),
  Bool(bool),
  Null,
}

impl Display for Literal {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Literal::Integer(i) => write!(f, "{}", i),
      Literal::Float(fl) => write!(f, "{}", fl),
      Literal::String(s) => write!(f, "{}", s),
      Literal::Bool(b) => write!(f, "{}", b),
      Literal::Null => write!(f, "null"),
    }
  }
}
