use thiserror::Error as ThisError;

use super::{span::Span, Error};

/// Specific types of errors that can occur.
#[derive(Debug, Clone, PartialEq, ThisError)]
pub enum ErrorKind {
  #[error("{0}")]
  RuntimeError(String),
  #[error("{0}")]
  SyntaxError(String),
  #[error("{0}")]
  TypeError(String),
  #[error("{0}")]
  ReferenceError(String),
  #[error("{0}")]
  ArgumentError(String),
}

impl ErrorKind {
  /// Returns the string representation of the error category.
  pub fn name(&self) -> &'static str {
    match self {
      ErrorKind::RuntimeError(_) => "RuntimeError",
      ErrorKind::SyntaxError(_) => "SyntaxError",
      ErrorKind::TypeError(_) => "TypeError",
      ErrorKind::ReferenceError(_) => "ReferenceError",
      ErrorKind::ArgumentError(_) => "ArgumentError",
    }
  }

  /// Wraps the ErrorKind with a span to create a full Error.
  pub fn at(self, span: Span) -> Error {
    Error::new(self, span)
  }
}
