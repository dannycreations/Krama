use krama_core::{
  AssignmentOperator, Error, ErrorKind, Expression, ExpressionKind,
  MatchPattern, ObjectKind, Span,
};

use crate::Interpreter;

impl<'ast> Interpreter<'ast> {
  /// Attempts to match an assignment expression as a pattern (e.g., Ok(v) = expr).
  /// Returns Ok(Some(bindings)) if it's a pattern assignment and it matched.
  /// Returns Ok(None) if it's NOT a pattern assignment OR it's a pattern assignment that failed.
  /// Used primarily in 'if' and 'while' conditions for elegant error handling.
  pub async fn try_match_assignment(
    &self,
    expression: &Expression<'ast>,
  ) -> Result<Option<Vec<(&'ast str, ObjectKind<'ast>)>>, Error<'ast>> {
    if let ExpressionKind::Assignment {
      left,
      operator: AssignmentOperator::Assign,
      right,
    } = &expression.kind
    {
      if let ExpressionKind::Call {
        function,
        arguments,
      } = &left.kind
      {
        if let ExpressionKind::Identifier(name) = &function.kind {
          // 1. Check if it's a Result pattern (Ok/Err).
          if (*name == "Ok" || *name == "Err") && arguments.len() == 1 {
            let right_val = self.eval_expression(right, None).await?;

            // Unwrap Return(Err) for pattern matching.
            let effective_val = if let ObjectKind::Return(inner) = &right_val {
              if let ObjectKind::Err(_) = inner {
                inner
              } else {
                &right_val
              }
            } else {
              &right_val
            };

            // 2. Verify variant match.
            let is_match = matches!(
              (*name, effective_val),
              ("Ok", ObjectKind::Ok(_)) | ("Err", ObjectKind::Err(_))
            );

            if is_match {
              let inner_val = match effective_val {
                ObjectKind::Ok(v) | ObjectKind::Err(v) => v,
                _ => unreachable!(),
              };

              // 3. Extract binding name from the pattern argument.
              if let ExpressionKind::Identifier(bind_name) = &arguments[0].kind
              {
                return Ok(Some(vec![(*bind_name, (*inner_val).clone())]));
              }
              return Ok(Some(Vec::new()));
            }
            // Pattern matched but variant was different (e.g. expected Ok, got Err).
            return Ok(None);
          }
        }
      }
    }
    Ok(None)
  }

  /// Evaluates a match pattern against a subject value.
  /// Returns Ok(Some(bindings)) if the pattern matches.
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
      // 1. Expression-based patterns (Literals, Result variants).
      (MatchPattern::Expression(expression), _) => {
        // Handle Result variant patterns: Ok(v) or Err(e)
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
                  ObjectKind::Ok(v) | ObjectKind::Err(v) => v,
                  _ => unreachable!(),
                };

                let arg = &arguments[0];
                if let ExpressionKind::Identifier(bind_name) = &arg.kind {
                  return Ok(Some(vec![(*bind_name, (*inner_val).clone())]));
                } else {
                  // Nested pattern matching (currently only direct value equality).
                  let arg_val = self.eval_expression(arg, None).await?;
                  if arg_val == **inner_val {
                    return Ok(Some(Vec::new()));
                  }
                }
              }
              return Ok(None);
            }
          }
        }

        // 2. Direct value matching (Equality).
        let pattern_val = self.eval_expression(expression, None).await?;
        if pattern_val == *subject {
          Ok(Some(Vec::new()))
        } else {
          Ok(None)
        }
      }
      // 3. Range patterns for integers.
      (MatchPattern::Range(start, end), ObjectKind::Integer(i)) => {
        let (start_val, end_val) = tokio::try_join!(
          self.eval_expression(start, None),
          self.eval_expression(end, None)
        )?;
        if let (ObjectKind::Integer(start), ObjectKind::Integer(end)) =
          (start_val, end_val)
        {
          if *i >= start && *i <= end {
            return Ok(Some(Vec::new()));
          }
          Ok(None)
        } else {
          Err(
            ErrorKind::TypeError(
              "Range pattern can only be used on integers".to_string(),
            )
            .at(span),
          )
        }
      }
      // 4. Range patterns for strings.
      (MatchPattern::Range(start, end), ObjectKind::String(s)) => {
        let (start_obj, end_obj) = tokio::try_join!(
          self.eval_expression(start, None),
          self.eval_expression(end, None)
        )?;
        if let (ObjectKind::String(start_str), ObjectKind::String(end_str)) =
          (start_obj, end_obj)
        {
          if *s >= start_str && *s <= end_str {
            return Ok(Some(Vec::new()));
          }
          Ok(None)
        } else {
          Err(
            ErrorKind::TypeError(
              "Range pattern can only be used on strings".to_string(),
            )
            .at(span),
          )
        }
      }
      // 5. Wildcard/Else pattern.
      (MatchPattern::Else, _) => Ok(Some(Vec::new())),
      _ => Ok(None),
    }
  }
}
