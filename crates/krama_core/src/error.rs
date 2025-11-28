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

impl std::error::Error for Error {}

#[derive(Debug, Clone, PartialEq, AsRefStr)]
#[strum(serialize_all = "PascalCase")]
pub enum ErrorKind {
  RuntimeError(String),
  SyntaxError(String),
  TypeError(String),
  ReferenceError(String),
  ArgumentError(String),
}

impl ErrorKind {
  pub fn message(&self) -> &str {
    match self {
      Self::RuntimeError(msg)
      | Self::SyntaxError(msg)
      | Self::TypeError(msg)
      | Self::ReferenceError(msg)
      | Self::ArgumentError(msg) => msg,
    }
  }
}

impl Display for ErrorKind {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    write!(f, "{}: {}", self.as_ref(), self.message())
  }
}
