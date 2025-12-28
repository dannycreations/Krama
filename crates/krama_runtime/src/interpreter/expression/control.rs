use std::sync::Arc;

use futures::try_join;
use krama_core::{
  AssignmentOperator, ErrorKind, ErrorResult, Expression, ExpressionKind,
  FunctionBody, Match, Object, ObjectResult, Pattern, Span, Type,
};

use crate::interpreter::Interpreter;

impl Interpreter {
  pub async fn eval_if_expression(
    &self,
    condition: &Expression,
    then_branch: &Expression,
    else_branch: Option<&Expression>,
    ty: Option<&Type>,
  ) -> ObjectResult {
    if let Some(bindings) = self.try_match_assignment(condition).await? {
      let stack = self.stack.clone();
      stack.write().push("if_binding".into(), None);
      for (name, val) in bindings {
        stack.write().define(name, val, false, false);
      }
      let result = self.eval_expression(then_branch, ty).await;
      stack.write().pop();
      return result;
    }

    let condition_val = self.eval_expression(condition, None).await?;
    if condition_val.is_truthy() {
      self.eval_expression(then_branch, ty).await
    } else if let Some(else_branch) = else_branch {
      self.eval_expression(else_branch, ty).await
    } else {
      Ok(Object::Void)
    }
  }

  pub async fn eval_match_expression(
    &self,
    subject: &Expression,
    arms: &[Match],
    span: Span,
  ) -> ObjectResult {
    let subject_val = self.eval_expression(subject, None).await?;
    let effective_val = subject_val.unwrap_return_err();

    for arm in arms {
      for pattern in &arm.patterns {
        if let Some(bindings) = self
          .eval_match_pattern(effective_val, pattern, span)
          .await?
        {
          if !bindings.is_empty() {
            self.stack.write().push("match_arm".into(), None);
            for (name, val) in &bindings {
              self.stack.write().define(
                name.clone(),
                val.clone(),
                false,
                false,
              );
            }
          }

          let result = match &arm.body {
            FunctionBody::Block(block) => {
              self.eval_block_statement_with_new_scope(block).await
            }
            FunctionBody::Expression(expression) => {
              self.eval_expression(expression, None).await
            }
          };

          if !bindings.is_empty() {
            self.stack.write().pop();
          }

          let result = result?;
          if result.is_control_signal() {
            if let Object::Return(_) = &result {
              return Ok(result);
            }
            return Ok(Object::Void);
          }
          return Ok(result);
        }
      }
    }
    Ok(Object::Void)
  }

  async fn eval_match_pattern<'s>(
    &'s self,
    subject: &'s Object,
    pattern: &'s Pattern,
    span: Span,
  ) -> ErrorResult<Option<Vec<(Arc<str>, Object)>>> {
    match (pattern, subject) {
      (Pattern::Expression(expression), _) => {
        self.match_pattern_internal(subject, expression, span).await
      }
      (Pattern::Range(start, end), Object::Integer(i)) => {
        let (start_val, end_val) = try_join!(
          self.eval_expression(start, None),
          self.eval_expression(end, None)
        )?;
        if let (Object::Integer(start), Object::Integer(end)) =
          (start_val, end_val)
        {
          if i >= &start && i <= &end {
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
      (Pattern::Range(start, end), Object::String(s)) => {
        let (start_obj, end_obj) = try_join!(
          self.eval_expression(start, None),
          self.eval_expression(end, None)
        )?;
        if let (Object::String(start_str), Object::String(end_str)) =
          (start_obj, end_obj)
        {
          if s.as_ref() >= start_str.as_ref() && s.as_ref() <= end_str.as_ref()
          {
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
      (Pattern::Else, _) => Ok(Some(Vec::new())),
      _ => Ok(None),
    }
  }

  pub async fn eval_result(
    &self,
    expr: &Expression,
    _span: Span,
  ) -> ObjectResult {
    let val = self.eval_expression(expr, None).await?;
    if let Object::Return(inner) = &val {
      if inner.is_result_err() {
        return Ok(inner.as_ref().clone());
      }
    }
    Ok(val)
  }

  pub async fn try_match_assignment(
    &self,
    expression: &Expression,
  ) -> ErrorResult<Option<Vec<(Arc<str>, Object)>>> {
    if let ExpressionKind::Assignment {
      left,
      operator: AssignmentOperator::Assign,
      right,
    } = &expression.kind
    {
      let right_val = self.eval_expression(right, None).await?;
      let effective_val = right_val.unwrap_return_err();

      return self
        .match_pattern_internal(effective_val, left, expression.span)
        .await;
    }
    Ok(None)
  }

  pub async fn match_pattern_internal(
    &self,
    subject: &Object,
    pattern_expr: &Expression,
    _span: Span,
  ) -> ErrorResult<Option<Vec<(Arc<str>, Object)>>> {
    if let ExpressionKind::Call {
      function,
      arguments,
    } = &pattern_expr.kind
    {
      if let ExpressionKind::Identifier(name) = &function.kind {
        if (name.as_ref() == "Ok" || name.as_ref() == "Err")
          && arguments.len() == 1
        {
          let is_match = matches!(
            (name.as_ref(), subject),
            ("Ok", Object::Ok(_)) | ("Err", Object::Err(_))
          );

          if is_match {
            let inner_val = match subject {
              Object::Ok(v) | Object::Err(v) => v,
              _ => unreachable!("is_match guaranteed this variant"),
            };

            let arg = &arguments[0];
            if let ExpressionKind::Identifier(bind_name) = &arg.kind {
              return Ok(Some(vec![(
                bind_name.clone(),
                inner_val.as_ref().clone(),
              )]));
            } else {
              let arg_val = self.eval_expression(arg, None).await?;
              if arg_val == *inner_val.as_ref() {
                return Ok(Some(Vec::new()));
              }
            }
          }
          return Ok(None);
        }
      }
    }

    let pattern_val = self.eval_expression(pattern_expr, None).await?;
    if pattern_val == *subject {
      Ok(Some(Vec::new()))
    } else {
      Ok(None)
    }
  }

  #[inline(always)]
  pub fn handle_loop_control(&self, result: Object) -> Option<Object> {
    if result.is_control_signal() {
      match result {
        Object::Break => Some(Object::Void),
        Object::Continue => None,
        Object::Return(inner) if inner.is_result_err() => None,
        _ => Some(result),
      }
    } else {
      None
    }
  }
}
