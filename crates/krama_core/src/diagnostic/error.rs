use ariadne::{Color, Label, Report, ReportKind, Source};
use thiserror::Error;

use super::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Error<'a> {
  pub kind: ErrorKind,
  pub span: Span,
  pub source: Option<&'a str>,
  pub file: Option<&'a str>,
}

impl<'a> Error<'a> {
  pub fn new(kind: ErrorKind, span: Span) -> Self {
    Self {
      kind,
      span,
      source: None,
      file: None,
    }
  }

  /// Attach source code and file path context to the error for reporting.
  pub fn with_context(mut self, source: &'a str, file: &'a str) -> Self {
    self.source = Some(source);
    self.file = Some(file);
    self
  }
}

#[derive(Debug, Clone, PartialEq, Error)]
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

/// Reports the error using Ariadne.
/// The error MUST have context attached via `with_context`.
pub fn report_error(error: Error<'_>) {
  let msg = error.kind.to_string();
  let kind = error.kind.name();
  let span = error.span;

  let file = error
    .file
    .expect("Error must have file context for reporting")
    .replace('\\', "/");
  let source = error
    .source
    .expect("Error must have source context for reporting");

  Report::build(
    ReportKind::Custom(kind, Color::Magenta),
    (&file, span.start..span.end),
  )
  .with_message(&msg)
  .with_label(
    Label::new((&file, span.start..span.end))
      .with_message(&msg)
      .with_color(Color::Red),
  )
  .finish()
  .print((&file, Source::from(source)))
  .unwrap();
}
