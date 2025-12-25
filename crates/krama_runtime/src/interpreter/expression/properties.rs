use indexmap::IndexMap;
use krama_core::{Error, ErrorKind, ErrorResult, Expression, ObjectKind};

use crate::interpreter::Interpreter;

impl Interpreter {
  /// Shared logic for evaluating property-based structures (Object, StructConstruction).
  pub async fn eval_properties(
    &self,
    properties: &[(Expression, Expression)],
  ) -> ErrorResult<IndexMap<String, ObjectKind>> {
    let mut fields = IndexMap::with_capacity(properties.len());
    for (key, value) in properties {
      let key_obj = self.eval_expression(key, None).await?;
      let ObjectKind::String(key_str) = key_obj else {
        return Err(Error::new(
          ErrorKind::TypeError("Expected string key".into()),
          key.span,
        ));
      };
      fields.insert(key_str, self.eval_expression(value, None).await?);
    }
    Ok(fields)
  }
}
