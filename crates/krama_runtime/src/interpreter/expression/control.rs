use std::borrow::Cow;

use krama_core::{
  Error, ErrorKind, Expression, ExpressionKind, FunctionBody, Match,
  MatchPattern, ObjectKind, Span, Type,
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
  ) -> Result<ObjectKind, Error> {
    // 1. Check if the condition is a pattern match (e.g., if (Ok(v) = expr)).
    if let Some(bindings) = self.try_match_assignment(condition).await? {
      let new_interpreter = self.new_enclosed();
      for (name, val) in bindings {
        new_interpreter
          .env_mut(condition.span)?
          .set(&name, val, false, false);
      }
      return new_interpreter.eval_expression(then_branch, kind).await;
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
  ) -> Result<ObjectKind, Error> {
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
          // 1. Prepare interpreter with bindings if necessary.
          let interpreter = if !bindings.is_empty() {
            let new_interp = self.new_enclosed();
            for (name, val) in bindings {
              new_interp.env_mut(span)?.set(&name, val, false, false);
            }
            Cow::Owned(new_interp)
          } else {
            Cow::Borrowed(self)
          };

          // 2. Execute arm body.
          let result = match &arm.body {
            FunctionBody::Block(block) => {
              interpreter
                .eval_block_statement_with_new_scope(block)
                .await?
            }
            FunctionBody::Expression(expression) => {
              interpreter
                .new_enclosed()
                .eval_expression(expression, None)
                .await?
            }
          };

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
  ) -> Result<Option<Vec<(String, ObjectKind)>>, Error> {
    match (pattern, subject) {
      // 1. Expression-based patterns (Result variants).
      (MatchPattern::Expression(expression), _) => {
        // Handle Result variant patterns: Ok(v) or Err(e)
        if let ExpressionKind::Call {
          function,
          arguments,
        } = &expression.kind
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
                  _ => unreachable!(),
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
