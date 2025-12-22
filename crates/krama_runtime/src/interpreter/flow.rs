use krama_core::{
  Error, ErrorKind, ExpressionKind, MatchPattern, ObjectKind, Span,
};

use crate::Interpreter;

impl<'ast> Interpreter<'ast> {
  pub async fn eval_match_pattern<'s>(
    &'s self,
    subject: &'s ObjectKind<'ast>,
    pattern: &'s MatchPattern<'ast>,
    span: Span,
  ) -> Result<Option<Vec<(&'ast str, ObjectKind<'ast>)>>, Error<'ast>>
  where
    'ast: 's,
  {
    match (pattern, subject) {
      (MatchPattern::Expression(expression), _) => {
        if let ExpressionKind::Call {
          function,
          arguments,
        } = &expression.kind
        {
          if let ExpressionKind::Identifier(name) = &function.kind {
            if (*name == "Ok" || *name == "Err") && arguments.len() == 1 {
              let is_match = matches!(
                (name, subject),
                (&"Ok", ObjectKind::Ok(_)) | (&"Err", ObjectKind::Err(_))
              );

              if is_match {
                let inner_val = match subject {
                  ObjectKind::Ok(v) => v,
                  ObjectKind::Err(v) => v,
                  _ => unreachable!(),
                };

                let arg = &arguments[0];
                if let ExpressionKind::Identifier(bind_name) = &arg.kind {
                  return Ok(Some(vec![(*bind_name, (*inner_val).clone())]));
                } else {
                  let arg_val = self.eval_expression(arg, None).await?;
                  if arg_val == **inner_val {
                    return Ok(Some(Vec::new()));
                  } else {
                    return Ok(None);
                  }
                }
              } else {
                return Ok(None);
              }
            }
          }
        }

        if let ExpressionKind::Literal(literal) = expression.kind {
          let pattern_val = self.eval_literal(literal)?;
          if pattern_val == *subject {
            Ok(Some(Vec::new()))
          } else {
            Ok(None)
          }
        } else {
          let pattern_val = self.eval_expression(expression, None).await?;
          if pattern_val == *subject {
            Ok(Some(Vec::new()))
          } else {
            Ok(None)
          }
        }
      }
      (MatchPattern::Range(start, end), ObjectKind::Integer(i)) => {
        let start_val = self.eval_expression(start, None).await?;
        let end_val = self.eval_expression(end, None).await?;
        if let (ObjectKind::Integer(start), ObjectKind::Integer(end)) =
          (start_val, end_val)
        {
          if *i >= start && *i <= end {
            Ok(Some(Vec::new()))
          } else {
            Ok(None)
          }
        } else {
          Err(Error::new(
            ErrorKind::TypeError(
              "Range pattern can only be used on integers".to_string(),
            ),
            span,
          ))
        }
      }
      (MatchPattern::Range(start, end), ObjectKind::String(s)) => {
        let start_obj = self.eval_expression(start, None).await?;
        let end_obj = self.eval_expression(end, None).await?;
        if let (ObjectKind::String(start_str), ObjectKind::String(end_str)) =
          (start_obj, end_obj)
        {
          if *s >= start_str && *s <= end_str {
            Ok(Some(Vec::new()))
          } else {
            Ok(None)
          }
        } else {
          Err(Error::new(
            ErrorKind::TypeError(
              "Range pattern can only be used on strings".to_string(),
            ),
            span,
          ))
        }
      }
      (MatchPattern::Else, _) => Ok(Some(Vec::new())),
      _ => Ok(None),
    }
  }
}
