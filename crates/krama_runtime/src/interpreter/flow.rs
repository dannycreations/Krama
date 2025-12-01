use krama_core::{
  ast::expression::{ExpressionKind, MatchPattern},
  error::{Error, ErrorKind},
  object::Object,
  span::Span,
};

use crate::interpreter::Interpreter;

impl<'ast> Interpreter<'ast> {
  pub(crate) async fn eval_match_pattern<'s>(
    &'s self,
    subject: &'s Object<'ast>,
    pattern: &'s MatchPattern<'ast>,
    span: Span<'ast>,
  ) -> Result<bool, Error<'ast>>
  where
    'ast: 's,
  {
    match (pattern, subject) {
      (MatchPattern::Expression(expression), _) => {
        if let ExpressionKind::Literal(literal) = expression.kind {
          let pattern = self.eval_literal(literal)?;
          Ok(pattern == *subject)
        } else {
          let pattern = self.eval_expression(expression, None).await?;
          Ok(pattern == *subject)
        }
      }
      (MatchPattern::Range(start, end), Object::Integer(i)) => {
        let start = self.eval_expression(start, None).await?;
        let end = self.eval_expression(end, None).await?;
        if let (Object::Integer(start), Object::Integer(end)) = (start, end) {
          Ok(*i >= start && *i <= end)
        } else {
          Err(Error::new(
            ErrorKind::TypeError(
              "Range pattern can only be used on integers".to_string(),
            ),
            span,
          ))
        }
      }
      (MatchPattern::Range(start, end), Object::String(s)) => {
        let start = self.eval_expression(start, None).await?;
        let end = self.eval_expression(end, None).await?;
        if let (Object::String(start), Object::String(end)) = (start, end) {
          let start_char = start.chars().next().unwrap();
          let end_char = end.chars().next().unwrap();
          let subject_char = s.chars().next().unwrap();
          Ok(subject_char >= start_char && subject_char <= end_char)
        } else {
          Err(Error::new(
            ErrorKind::TypeError(
              "Range pattern can only be used on strings".to_string(),
            ),
            span,
          ))
        }
      }
      (MatchPattern::Else, _) => Ok(true),
      _ => Ok(false),
    }
  }
}
