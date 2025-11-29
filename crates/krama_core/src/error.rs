use std::fmt::{Display, Formatter};

use strum_macros::AsRefStr;

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
