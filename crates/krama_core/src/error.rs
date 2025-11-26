use std::fmt;

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

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
  RuntimeError(String),
  SyntaxError(String),
  TypeError(String),
  ReferenceError(String),
}

impl fmt::Display for ErrorKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      ErrorKind::RuntimeError(msg) => write!(f, "RuntimeError: {}", msg),
      ErrorKind::SyntaxError(msg) => write!(f, "SyntaxError: {}", msg),
      ErrorKind::TypeError(msg) => write!(f, "TypeError: {}", msg),
      ErrorKind::ReferenceError(msg) => write!(f, "ReferenceError: {}", msg),
    }
  }
}
