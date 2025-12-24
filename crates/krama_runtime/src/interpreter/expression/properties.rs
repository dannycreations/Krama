use indexmap::IndexMap;
use krama_core::{Error, ErrorKind, Expression, ObjectKind};

use crate::interpreter::Interpreter;

impl<'ast> Interpreter<'ast> {
  /// Shared logic for evaluating property-based structures (Object, StructConstruction).
  pub(crate) async fn eval_properties(
    &self,
    properties: &[(Expression<'ast>, Expression<'ast>)],
  ) -> Result<IndexMap<&'ast str, ObjectKind<'ast>>, Error<'ast>> {
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
