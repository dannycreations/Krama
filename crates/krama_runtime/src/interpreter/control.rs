use krama_core::{
  AssignmentOperator, ErrorResult, Expression, ExpressionKind, ObjectKind,
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
      // Patterns look like function calls (e.g., Ok(v)).
      if let ExpressionKind::Call {
        function,
        arguments,
      } = &left.kind
      {
        if let ExpressionKind::Identifier(name) = &function.kind {
          // 1. Check if it's a Result pattern (Ok/Err).
          if (name == "Ok" || name == "Err") && arguments.len() == 1 {
            let right_val = self.eval_expression(right, None).await?;

            // Unwrap Return(Err) for pattern matching using centralized unwrap_return_err.
            // This ensures that implicit error propagation doesn't break pattern matching.
            let effective_val = right_val.unwrap_return_err();

            // 2. Verify variant match.
            let is_match = matches!(
              (name.as_str(), effective_val),
              ("Ok", ObjectKind::Ok(_)) | ("Err", ObjectKind::Err(_))
            );

            if is_match {
              let inner_val = match effective_val {
                ObjectKind::Ok(v) | ObjectKind::Err(v) => v,
                _ => unreachable!("is_match guaranteed this variant"),
              };

              // 3. Extract binding name from the pattern argument.
              if let ExpressionKind::Identifier(bind_name) = &arguments[0].kind
              {
                return Ok(Some(vec![(
                  bind_name.to_string(),
                  *(*inner_val).clone(),
                )]));
              }
              // Pattern matched but no variable was bound (e.g., Ok(_)).
              return Ok(Some(Vec::new()));
            }
            // Pattern matched syntactically but variant was different (e.g. expected Ok, got Err).
            return Ok(None);
          }
        }
      }
    }
    Ok(None)
  }
}
