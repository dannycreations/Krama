use std::sync::Arc;

use futures::future::try_join_all;
use indexmap::IndexMap;
use krama_core::{
  BinaryOperator, Error, ErrorKind, Expression, ExpressionKind, ObjectKind,
  ObjectResult, Span, Type, TypeKind,
};

use crate::interpreter::Interpreter;

impl Interpreter {
  pub async fn eval_identifier(
    &self,
    expression: &Expression,
    name: &str,
    span: Span,
  ) -> ObjectResult {
    if let Some(distance) = self.get_resolved_distance(expression) {
      if let Some(value) = self.get_at(distance, name) {
        return Ok(value);
      }
    }
    self.stack.read().get(name).ok_or_else(|| {
      Error::new(
        ErrorKind::ReferenceError(format!("'{}' is not defined", name)),
        span,
      )
    })
  }

  pub fn get_this(&self, span: Span) -> ObjectResult {
    self.stack.read().get("this").ok_or_else(|| {
      Error::new(
        ErrorKind::ReferenceError(
          "'this' is not defined in the current scope".into(),
        ),
        span,
      )
    })
  }

  pub async fn eval_collection(
    &self,
    elements: &[Expression],
    kind_hint: Option<&Type>,
    span: Span,
  ) -> ObjectResult {
    let el_hint = kind_hint.and_then(|hint| {
      if let TypeKind::Array { element, .. } = &hint.kind {
        Some(element.as_ref())
      } else {
        None
      }
    });

    let results =
      try_join_all(elements.iter().map(|e| self.eval_expression(e, el_hint)))
        .await?;

    Ok(match kind_hint.map(|h| &h.kind) {
      Some(TypeKind::Array { .. }) => self.heap.write().alloc_array(
        results,
        kind_hint.unwrap().clone(),
        false,
      ),
      Some(TypeKind::Tuple(_)) => self.heap.write().alloc_tuple(results),
      _ if results.is_empty() => self.heap.write().alloc_array(
        Vec::new(),
        Type::new(
          TypeKind::Array {
            element: Box::new(Type::new(TypeKind::Void, span)),
            size: None,
          },
          span,
        ),
        false,
      ),
      _ => self.heap.write().alloc_tuple(results),
    })
  }

  pub async fn eval_object_literal(
    &self,
    properties: &[(Expression, Expression)],
  ) -> ObjectResult {
    let object = self.eval_properties(properties).await?;
    Ok(self.heap.write().alloc_object(object, None, false))
  }

  pub async fn eval_properties(
    &self,
    properties: &[(Expression, Expression)],
  ) -> crate::ErrorResult<IndexMap<Arc<str>, ObjectKind>> {
    let mut fields = IndexMap::with_capacity(properties.len());
    for (key_expr, value_expr) in properties {
      let key_str = if let ExpressionKind::Identifier(name) = &key_expr.kind {
        name.clone()
      } else {
        let key_obj = self.eval_expression(key_expr, None).await?;
        if let ObjectKind::String(s) = key_obj {
          s
        } else {
          return Err(Error::new(
            ErrorKind::TypeError(
              "Expected string key or identifier for property".into(),
            ),
            key_expr.span,
          ));
        }
      };
      fields.insert(key_str, self.eval_expression(value_expr, None).await?);
    }
    Ok(fields)
  }

  pub fn eval_unary_expression(
    &self,
    operator: krama_core::UnaryOperator,
    right: ObjectKind,
    span: Span,
  ) -> ObjectResult {
    right.unary_op(operator).map_err(|k| k.at(span))
  }

  pub async fn eval_binary_expression(
    &self,
    left: &Expression,
    operator: BinaryOperator,
    right: &Expression,
    span: Span,
  ) -> ObjectResult {
    if matches!(
      operator,
      BinaryOperator::LogicalOr | BinaryOperator::LogicalAnd
    ) {
      let left_val = self.eval_expression(left, None).await?;
      if left_val.is_control_signal() {
        return Ok(left_val);
      }
      if left_val.is_truthy() == (operator == BinaryOperator::LogicalOr) {
        return Ok(left_val);
      }
      return self.eval_expression(right, None).await;
    }

    let (l, r) = futures::try_join!(
      self.eval_expression(left, None),
      self.eval_expression(right, None)
    )?;
    l.binary_op(operator, &r).map_err(|k| k.at(span))
  }
}
