use ariadne::{Color, Label, Report, ReportKind, Source};

mod kind;
mod span;

pub use kind::*;
pub use span::*;

pub type ErrorResult<T = ()> = Result<T, Error>;
pub type ErrorKindResult<T = ()> = Result<T, ErrorKind>;

/// Represents a diagnostic error in the compiler/interpreter.
#[derive(Debug, Clone, PartialEq)]
pub struct Error {
  pub kind: ErrorKind,
  pub span: Span,
  pub source: Option<String>,
  pub file: Option<String>,
}

impl Error {
  /// Creates a new diagnostic error without context.
  pub fn new(kind: ErrorKind, span: Span) -> Self {
    Self {
      kind,
      span,
      source: None,
      file: None,
    }
  }

  /// Attach source code and file path context to the error for reporting.
  pub fn with_context(mut self, source: &str, file: &str) -> Self {
    self.source = Some(source.to_string());
    self.file = Some(file.to_string());
    self
  }

  /// Reports the error using Ariadne.
  /// The error MUST have context attached via `with_context`.
  pub fn report(self) {
    let msg = self.kind.to_string();
    let kind_name = self.kind.name();
    let span = self.span;

    let file = self
      .file
      .expect("Error must have file context for reporting")
      .replace('\\', "/");
    let source = self
      .source
      .expect("Error must have source context for reporting");

    Report::build(
      ReportKind::Custom(kind_name, Color::Magenta),
      (&file, span.start..span.end),
    )
    .with_message(&msg)
    .with_label(
      Label::new((&file, span.start..span.end))
        .with_message(&msg)
        .with_color(Color::Red),
    )
    .finish()
    .print((&file, Source::from(&source)))
    .unwrap();
  }
}
