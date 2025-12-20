use ariadne::{Color, Label, Report, ReportKind, Source};
use thiserror::Error;

use super::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Error<'a> {
  pub kind: ErrorKind,
  pub span: Span<'a>,
}

impl<'a> Error<'a> {
  pub fn new(kind: ErrorKind, span: Span<'a>) -> Self {
    Self { kind, span }
  }
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum ErrorKind {
  #[error("RuntimeError: {0}")]
  RuntimeError(String),
  #[error("SyntaxError: {0}")]
  SyntaxError(String),
  #[error("TypeError: {0}")]
  TypeError(String),
  #[error("ReferenceError: {0}")]
  ReferenceError(String),
  #[error("ArgumentError: {0}")]
  ArgumentError(String),
}

impl ErrorKind {
  pub fn name(&self) -> &'static str {
    match self {
      ErrorKind::RuntimeError(_) => "RuntimeError",
      ErrorKind::SyntaxError(_) => "SyntaxError",
      ErrorKind::TypeError(_) => "TypeError",
      ErrorKind::ReferenceError(_) => "ReferenceError",
      ErrorKind::ArgumentError(_) => "ArgumentError",
    }
  }
}

pub fn report_error(error: Error<'_>) {
  let msg = error.kind.to_string();
  let kind = error.kind.name();
  let span = error.span;
  let file = span.file.unwrap_or("<unknown>");

  Report::build(
    ReportKind::Custom(kind, Color::Magenta),
    (file, span.start..span.end),
  )
  .with_message(&msg)
  .with_label(
    Label::new((file, span.start..span.end))
      .with_message(&msg)
      .with_color(Color::Red),
  )
  .finish()
  .print((file, Source::from(span.source.unwrap_or_default())))
  .unwrap();
}
