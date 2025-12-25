use indexmap::IndexMap;
use krama_core::{
  Error, ErrorKind, Expression, ObjectKind, ObjectResult, Span,
};

use crate::interpreter::{types::check_type, Interpreter};

impl Interpreter {
  /// Evaluates a struct construction expression.
  pub async fn eval_struct_construction(
    &self,
    properties: &[(Expression, Expression)],
    span: Span,
  ) -> ObjectResult {
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
      let value = match fields.get(field.name.as_ref()) {
        Some(val) => val.clone(),
        None => match &field.default {
          Some(default) => self.eval_expression(default, None).await?,
          None => {
            return Err(Error::new(
              ErrorKind::TypeError(format!("Missing field '{}'", field.name)),
              span,
            ))
          }
        },
      };
      check_type(&field.kind, &value)?;
      final_fields.insert(field.name.clone(), value);
    }

    Ok(
      self
        .heap
        .write()
        .alloc_object(final_fields, Some(definition), false),
    )
  }
}
