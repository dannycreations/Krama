use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
use krama_core::{error::ErrorKind, span::Span};

pub fn report_error(
  default_file: &str,
  default_content: &str,
  span: Span,
  kind: ErrorKind,
) {
  let msg = kind.to_string();
  let kind_name = kind.as_ref();

  let file = span.file.unwrap_or(default_file);
  let content = span.source.unwrap_or(default_content);

  Report::build(
    ReportKind::Custom(kind_name, Color::Magenta),
    (file, span.start..span.end),
  )
  .with_message(msg.clone().fg(Color::White))
  .with_label(
    Label::new((file, span.start..span.end))
      .with_message(msg.fg(Color::White))
      .with_color(Color::Red),
  )
  .finish()
  .print((file, Source::from(content)))
  .unwrap();
}
