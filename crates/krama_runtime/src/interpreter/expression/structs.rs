use indexmap::IndexMap;
use krama_core::{Error, ErrorKind, Expression, ObjectKind, Span};
use parking_lot::RwLock;

use crate::interpreter::{types::check_type, Interpreter};

impl<'ast> Interpreter<'ast> {
  /// Evaluates a struct construction expression.
  pub async fn eval_struct_construction(
    &self,
    properties: &[(Expression<'ast>, Expression<'ast>)],
    span: Span,
  ) -> Result<ObjectKind<'ast>, Error<'ast>> {
    let this_obj = self.get_this(span)?;
    let ObjectKind::Struct(definition) = this_obj else {
      return Err(Error::new(
        ErrorKind::TypeError(format!(
          "'this' is not a struct definition, found {}",
          this_obj.type_name()
        )),
        span,
      ));
    };

    let fields = self.eval_properties(properties).await?;

    // Validate and apply default values for missing fields
    let mut final_fields = IndexMap::with_capacity(definition.fields.len());
    for field in &definition.fields {
      let value = if let Some(val) = fields.get(field.name) {
        val.clone()
      } else if let Some(default) = field.default {
        self.eval_expression(default, None).await?
      } else {
        return Err(Error::new(
          ErrorKind::TypeError(format!("Missing field '{}'", field.name)),
          span,
        ));
      };
      check_type(&field.kind, &value)?;
      final_fields.insert(field.name, value);
    }

    Ok(ObjectKind::Object {
      properties: self.arena.alloc(RwLock::new(final_fields)),
      definition: Some(definition),
      constant: false,
    })
  }
}
