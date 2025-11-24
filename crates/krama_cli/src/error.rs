use ariadne::Color;
use ariadne::Fmt;
use ariadne::Label;
use ariadne::Report;
use ariadne::ReportKind;
use ariadne::Source;
use krama_core::error::Error;

pub fn report_error(file_path: &str, content: &str, error: Error) {
  let (msg, span) = (error.kind.to_string(), error.span);
  Report::build(ReportKind::Error, (file_path, span.start..span.end))
    .with_message(msg.clone().fg(Color::White))
    .with_label(
      Label::new((file_path, span.start..span.end))
        .with_message(msg.fg(Color::Red))
        .with_color(Color::Red),
    )
    .finish()
    .print((file_path, Source::from(content)))
    .unwrap();
}
