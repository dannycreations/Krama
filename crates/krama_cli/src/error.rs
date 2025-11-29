use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
use krama_core::error::Error;

pub fn report_error(
  default_file_path: &str,
  default_content: &str,
  error: Error,
) {
  let msg = error.kind.message();
  let span = error.span;
  let kind_name = error.kind.as_ref();

  let file_path = error.file_path.as_deref().unwrap_or(default_file_path);
  let content = error.source.as_deref().unwrap_or(default_content);

  Report::build(
    ReportKind::Custom(kind_name, Color::Magenta),
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
