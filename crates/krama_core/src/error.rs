use std::fmt::{Display, Formatter, Result};

use strum_macros::AsRefStr;

use crate::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
  pub span: Span,
  pub kind: ErrorKind,
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter) -> Result {
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
  ArgumentError(String),
}

impl Display for ErrorKind {
  fn fmt(&self, f: &mut Formatter) -> Result {
    let (variant, msg) = match self {
      ErrorKind::RuntimeError(msg) => (self.as_ref(), msg),
      ErrorKind::SyntaxError(msg) => (self.as_ref(), msg),
      ErrorKind::TypeError(msg) => (self.as_ref(), msg),
      ErrorKind::ReferenceError(msg) => (self.as_ref(), msg),
      ErrorKind::ArgumentError(msg) => (self.as_ref(), msg),
    };
    write!(f, "{}: {}", variant, msg)
  }
}
