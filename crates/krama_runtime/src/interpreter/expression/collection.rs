use std::vec::Vec;

use futures::future::try_join_all;
use krama_core::{Error, Expression, ObjectKind, Span, Type, TypeKind};

use crate::interpreter::Interpreter;

impl Interpreter {
  /// Evaluates a collection literal (array or tuple).
  pub async fn eval_collection(
    &self,
    elements: &[Expression],
    kind_hint: Option<&Type>,
    span: Span,
  ) -> Result<ObjectKind, Error> {
    // 1. Determine element type hint from the parent collection hint.
    let el_hint = kind_hint.and_then(|hint| {
      if let TypeKind::Array { element, .. } = &hint.kind {
        Some(element.as_ref())
      } else {
        None
      }
    });

    // 2. Evaluate all elements concurrently.
    let results = if elements.is_empty() {
      Vec::new()
    } else {
      try_join_all(elements.iter().map(|e| self.eval_expression(e, el_hint)))
        .await?
    };

    // 3. Construct the specific collection type if a hint is present.
    if let Some(hint) = kind_hint {
      match &hint.kind {
        TypeKind::Array { .. } => {
          return Ok(self.heap.write().alloc_array(
            results,
            hint.clone(),
            false,
          ));
        }
        TypeKind::Tuple(_) => {
          return Ok(self.heap.write().alloc_tuple(results));
        }
        _ => {}
      }
    }

    // 4. Default to Array if empty, or Tuple if non-empty and no hint is available.
    if results.is_empty() {
      Ok(self.heap.write().alloc_array(
        Vec::new(),
        Type::new(
          TypeKind::Array {
            element: Box::new(Type::new(TypeKind::Void, span)),
            size: None,
          },
          span,
        ),
        false,
      ))
    } else {
      Ok(self.heap.write().alloc_tuple(results))
    }
  }

  /// Evaluates an object literal expression by resolving its properties.
  pub async fn eval_object_literal(
    &self,
    properties: &[(Expression, Expression)],
  ) -> Result<ObjectKind, Error> {
    let object = self.eval_properties(properties).await?;
    Ok(self.heap.write().alloc_object(object, None, false))
  }
}
