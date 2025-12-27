use krama_core::{
  Error, ErrorKind, Expression, ForBinding, ObjectKind, ObjectResult, Span,
  Type,
};

use crate::interpreter::{
  types::{check_type, resolve_type},
  Interpreter,
};

impl Interpreter {
  pub async fn eval_and_check_type(
    &self,
    expr: &Expression,
    kind_hint: Option<&Type>,
  ) -> ObjectResult {
    let resolved = kind_hint.map(|k| resolve_type(self, k)).transpose()?;
    let value = self.eval_expression(expr, resolved.as_ref()).await?;
    if let Some(kind) = &resolved {
      check_type(kind, &value)?;
    }
    Ok(value)
  }

  pub fn collect_iterable_elements(
    &self,
    iterable: &ObjectKind,
    binding: &ForBinding,
    span: Span,
  ) -> Result<Vec<ObjectKind>, Error> {
    match iterable {
      ObjectKind::Array { elements, .. } => Ok(elements.read().to_vec()),
      ObjectKind::Tuple(elements) => Ok(elements.as_ref().to_vec()),
      ObjectKind::String(s) => Ok(
        s.chars()
          .map(|c| ObjectKind::String(c.to_string().into()))
          .collect(),
      ),
      ObjectKind::Object { properties, .. } => {
        let props = properties.read();
        let mut yields = Vec::with_capacity(props.len());

        match binding {
          ForBinding::Array(bindings) if bindings.len() == 2 => {
            for (k, v) in props.iter() {
              let elements = vec![ObjectKind::String(k.clone()), v.clone()];
              yields.push(self.heap.write().alloc_tuple(elements));
            }
          }
          _ => {
            for k in props.keys() {
              yields.push(ObjectKind::String(k.clone()));
            }
          }
        }
        Ok(yields)
      }
      _ => Err(Error::new(
        ErrorKind::TypeError(format!(
          "Expected array, tuple, string or object for for..in loop, found {}",
          iterable.type_name()
        )),
        span,
      )),
    }
  }
}
