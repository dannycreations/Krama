use krama_core::{
  ErrorKind, ErrorResult, Expression, FunctionBody, Match, MatchPattern,
  ObjectKind, ObjectResult, Span, Type,
};

use crate::Interpreter;

impl Interpreter {
  /// Evaluates an 'if' expression, including pattern matching support.
  pub async fn eval_if_expression(
    &self,
    condition: &Expression,
    then_branch: &Expression,
    else_branch: Option<&Expression>,
    kind: Option<&Type>,
  ) -> ObjectResult {
    // 1. Check if the condition is a pattern match (e.g., if (Ok(v) = expr)).
    if let Some(bindings) = self.try_match_assignment(condition).await? {
      // Push a scope for the bindings
      let stack = self.stack.clone();
      stack.write().push("if_binding".to_string(), None);

      for (name, val) in bindings {
        stack.write().define(name, val, false, false);
      }

      let result = self.eval_expression(then_branch, kind).await;
      stack.write().pop();
      return result;
    }

    // 2. Fallback to normal truthy evaluation.
    let condition_val = self.eval_expression(condition, None).await?;
    if condition_val.is_truthy() {
      self.eval_expression(then_branch, kind).await
    } else if let Some(else_branch) = else_branch {
      self.eval_expression(else_branch, kind).await
    } else {
      Ok(ObjectKind::Void)
    }
  }

  /// Evaluates a 'match' expression by iterating through patterns in each arm.
  pub async fn eval_match_expression(
    &self,
    subject: &Expression,
    arms: &[Match],
    span: Span,
  ) -> ObjectResult {
    let subject_val = self.eval_expression(subject, None).await?;

    // Use centralized unwrap_return_err to simplify error handling logic.
    let effective_val = subject_val.unwrap_return_err();

    // Iterate through each arm and its patterns.
    for arm in arms {
      for pattern in &arm.patterns {
        if let Some(bindings) = self
          .eval_match_pattern(effective_val, pattern, span)
          .await?
        {
          // 1. Prepare bindings if necessary.
          if !bindings.is_empty() {
            self.stack.write().push("match_arm".to_string(), None);
            for (name, val) in &bindings {
              self.stack.write().define(
                name.clone(),
                val.clone(),
                false,
                false,
              );
            }
          }

          // 2. Execute arm body.
          let result = match &arm.body {
            FunctionBody::Block(block) => {
              // eval_block_statement_with_new_scope will push another scope, which is fine
              self.eval_block_statement_with_new_scope(block).await
            }
            FunctionBody::Expression(expression) => {
              // Expression bodies share the current scope (which includes bindings)
              self.eval_expression(expression, None).await
            }
          };

          if !bindings.is_empty() {
            self.stack.write().pop();
          }

          let result = result?;

          // 3. Handle Return signals and control flow.
          if result.is_control_signal() {
            if let ObjectKind::Return(_) = &result {
              return Ok(result);
            }
            // Break/Continue in a match expression resolve to Void to allow the outer loop to handle the signal.
            return Ok(ObjectKind::Void);
          }

          return Ok(result);
        }
      }
    }

    Ok(ObjectKind::Void)
  }

  /// Evaluates a match pattern against a subject value.
  /// Returns Ok(Some(bindings)) if the pattern matches.
  async fn eval_match_pattern<'s>(
    &'s self,
    subject: &'s ObjectKind,
    pattern: &'s MatchPattern,
    span: Span,
  ) -> ErrorResult<Option<Vec<(String, ObjectKind)>>> {
    match (pattern, subject) {
      // 1. Expression-based patterns.
      (MatchPattern::Expression(expression), _) => {
        self.match_pattern_internal(subject, expression, span).await
      }
      // 2. Range patterns for integers.
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
