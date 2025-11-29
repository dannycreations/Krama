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

impl Display for ErrorKind {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    match self {
      ErrorKind::RuntimeError(msg)
      | ErrorKind::SyntaxError(msg)
      | ErrorKind::TypeError(msg)
      | ErrorKind::ReferenceError(msg)
      | ErrorKind::ArgumentError(msg) => {
        write!(f, "{}: {}", self.as_ref(), msg)
      }
    }
  }
}
