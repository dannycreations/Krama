use std::fmt;

use strum_macros::AsRefStr;

use crate::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
  pub span: Span,
  pub kind: ErrorKind,
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.kind)
  }
}

#[derive(Debug, Clone, PartialEq, AsRefStr)]
#[strum(serialize_all = "PascalCase")]
pub enum ErrorKind {
  RuntimeError(String),
  SyntaxError(String),
  TypeError(String),
  ReferenceError(String),
}

impl fmt::Display for ErrorKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      ErrorKind::RuntimeError(msg) => write!(f, "{}", msg),
      ErrorKind::SyntaxError(msg) => write!(f, "{}", msg),
      ErrorKind::TypeError(msg) => write!(f, "{}", msg),
      ErrorKind::ReferenceError(msg) => write!(f, "{}", msg),
    }
  }
}
