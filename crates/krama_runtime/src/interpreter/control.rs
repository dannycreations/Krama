use krama_core::{
  AssignmentOperator, ErrorResult, Expression, ExpressionKind, ObjectKind, Span,
};

use crate::Interpreter;

impl Interpreter {
  /// Attempts to match an assignment expression as a pattern (e.g., Ok(v) = expr).
  /// Returns Ok(Some(bindings)) if it's a pattern assignment and it matched.
  /// Returns Ok(None) if it's NOT a pattern assignment OR it's a pattern assignment that failed.
  /// Used primarily in 'if' and 'while' conditions for elegant error handling.
  pub async fn try_match_assignment(
    &self,
    expression: &Expression,
  ) -> ErrorResult<Option<Vec<(String, ObjectKind)>>> {
    // We only care about simple assignments for pattern matching.
    if let ExpressionKind::Assignment {
      left,
      operator: AssignmentOperator::Assign,
      right,
    } = &expression.kind
    {
      let right_val = self.eval_expression(right, None).await?;
      // Unwrap Return(Err) for pattern matching using centralized unwrap_return_err.
      // This ensures that implicit error propagation doesn't break pattern matching.
      let effective_val = right_val.unwrap_return_err();

      return self
        .match_pattern_internal(effective_val, left, expression.span)
        .await;
    }
    Ok(None)
  }

  /// Internal helper to match a value against a pattern expression.
  /// Centralizes pattern matching logic for 'if let', 'while let', and 'match'.
  pub async fn match_pattern_internal(
    &self,
    subject: &ObjectKind,
    pattern_expr: &Expression,
    _span: Span,
  ) -> ErrorResult<Option<Vec<(String, ObjectKind)>>> {
    // Handle Result variant patterns: Ok(v) or Err(e)
    if let ExpressionKind::Call {
      function,
      arguments,
    } = &pattern_expr.kind
    {
      if let ExpressionKind::Identifier(name) = &function.kind {
        if (name == "Ok" || name == "Err") && arguments.len() == 1 {
          let is_match = matches!(
            (name.as_str(), subject),
            ("Ok", ObjectKind::Ok(_)) | ("Err", ObjectKind::Err(_))
          );

          if is_match {
            let inner_val = match subject {
              ObjectKind::Ok(v) | ObjectKind::Err(v) => v,
              _ => unreachable!("is_match guaranteed this variant"),
            };

            let arg = &arguments[0];
            if let ExpressionKind::Identifier(bind_name) = &arg.kind {
              return Ok(Some(vec![(
                bind_name.clone(),
                *(*inner_val).clone(),
              )]));
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

    // Direct value matching (Equality).
    let pattern_val = self.eval_expression(pattern_expr, None).await?;
    if pattern_val == *subject {
      Ok(Some(Vec::new()))
    } else {
      Ok(None)
    }
  }
}
