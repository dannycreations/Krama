use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
use krama_core::error::Error;

pub fn report_error(file_path: &str, content: &str, error: Error) {
  let msg = error.kind.message();
  let span = error.span;
  let kind_name = error.kind.as_ref();

  Report::build(
    ReportKind::Custom(kind_name, Color::Red),
    (file_path, span.start..span.end),
  )
  .with_message(msg.fg(Color::White))
  .with_label(
    Label::new((file_path, span.start..span.end))
      .with_message(msg.fg(Color::White))
      .with_color(Color::Red),
  )
  .finish()
  .print((file_path, Source::from(content)))
  .unwrap();
}
