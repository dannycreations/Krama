use std::fmt::{Display, Formatter};

use ariadne::{Color, Label, Report, ReportKind, Source};
use strum_macros::AsRefStr;

use crate::span::Span;

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

pub fn report_error(error: Error<'_>) {
  let msg = error.kind.to_string();
  let kind_name = error.kind.as_ref();
  let span = error.span;

  let file = span.file.unwrap_or("<unknown>");
  let content = span.source.unwrap_or("");

  Report::build(
    ReportKind::Custom(kind_name, Color::Magenta),
    (file, span.start..span.end),
  )
  .with_message(&msg)
  .with_label(
    Label::new((file, span.start..span.end))
      .with_message(&msg)
      .with_color(Color::Red),
  )
  .finish()
  .print((file, Source::from(content)))
  .unwrap();
}
