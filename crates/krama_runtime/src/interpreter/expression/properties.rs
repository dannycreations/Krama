use std::sync::Arc;

use indexmap::IndexMap;
use krama_core::{
  Error, ErrorKind, ErrorResult, Expression, ExpressionKind, ObjectKind,
};

use crate::interpreter::Interpreter;

impl Interpreter {
  /// Shared logic for evaluating property-based structures (Object, StructConstruction).
  /// Optimized to pre-allocate and avoid intermediate ObjectKind::String clones.
  pub async fn eval_properties(
    &self,
    properties: &[(Expression, Expression)],
  ) -> ErrorResult<IndexMap<Arc<str>, ObjectKind>> {
    let mut fields = IndexMap::with_capacity(properties.len());
    for (key_expr, value_expr) in properties {
      // Fast-path: If the key is an identifier, we use it directly as the key.
      // This is common in object literals: { name: "value" }
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
}
