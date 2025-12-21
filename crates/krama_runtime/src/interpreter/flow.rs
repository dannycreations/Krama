use krama_core::{
  Error, ErrorKind, ExpressionKind, MatchPattern, Object, Span,
};

use crate::Interpreter;

impl<'ast> Interpreter<'ast> {
  pub async fn eval_match_pattern<'s>(
    &'s self,
    subject: &'s Object<'ast>,
    pattern: &'s MatchPattern<'ast>,
    span: Span,
  ) -> Result<bool, Error<'ast>>
  where
    'ast: 's,
  {
    match (pattern, subject) {
      (MatchPattern::Expression(expression), _) => {
        if let ExpressionKind::Literal(literal) = expression.kind {
          let pattern_val = self.eval_literal(literal)?;
          Ok(pattern_val == *subject)
        } else {
          let pattern_val = self.eval_expression(expression, None).await?;
          Ok(pattern_val == *subject)
        }
      }
      (MatchPattern::Range(start, end), Object::Integer(i)) => {
        let start_val = self.eval_expression(start, None).await?;
        let end_val = self.eval_expression(end, None).await?;
        if let (Object::Integer(start), Object::Integer(end)) =
          (start_val, end_val)
        {
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
        let start_obj = self.eval_expression(start, None).await?;
        let end_obj = self.eval_expression(end, None).await?;
        if let (Object::String(start_str), Object::String(end_str)) =
          (start_obj, end_obj)
        {
          Ok(*s >= start_str && *s <= end_str)
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
