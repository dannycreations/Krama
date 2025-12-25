use std::vec::Vec;

use futures::future::try_join_all;
use krama_core::{Expression, ObjectResult, Span, Type, TypeKind};

use crate::interpreter::Interpreter;

impl Interpreter {
  /// Evaluates a collection literal (array or tuple).
  pub async fn eval_collection(
    &self,
    elements: &[Expression],
    kind_hint: Option<&Type>,
    span: Span,
  ) -> ObjectResult {
    // 1. Determine element type hint from the parent collection hint.
    let el_hint = kind_hint.and_then(|hint| {
      if let TypeKind::Array { element, .. } = &hint.kind {
        Some(element.as_ref())
      } else {
        None
      }
    });

    // 2. Evaluate all elements concurrently.
    let results =
      try_join_all(elements.iter().map(|e| self.eval_expression(e, el_hint)))
        .await?;

    // 3. Construct the specific collection type based on hint or defaults.
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

  /// Evaluates an object literal expression by resolving its properties.
  pub async fn eval_object_literal(
    &self,
    properties: &[(Expression, Expression)],
  ) -> ObjectResult {
    let object = self.eval_properties(properties).await?;
    Ok(self.heap.write().alloc_object(object, None, false))
  }
}
