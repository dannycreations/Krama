use krama_core::{
  Error, ErrorKind, Expression, Iteration, Object, ObjectResult, Span, Type,
};

use crate::interpreter::{
  types::{check_type, resolve_type},
  Interpreter,
};

impl Interpreter {
  pub async fn eval_and_check_type(
    &self,
    expr: &Expression,
    ty: Option<&Type>,
  ) -> ObjectResult {
    let resolved = ty.map(|k| resolve_type(self, k)).transpose()?;
    let value = self.eval_expression(expr, resolved.as_ref()).await?;
    if let Some(ty) = &resolved {
      check_type(ty, &value)?;
    }
    Ok(value)
  }

  pub fn collect_iterable_elements(
    &self,
    iterable: &Object,
    binding: &Iteration,
    span: Span,
  ) -> Result<Vec<Object>, Error> {
    match iterable {
      Object::Array { elements, .. } => Ok(elements.read().to_vec()),
      Object::Tuple(elements) => Ok(elements.as_ref().to_vec()),
      Object::String(s) => Ok(
        s.chars()
          .map(|c| Object::String(c.to_string().into()))
          .collect(),
      ),
      Object::Object { properties, .. } => {
        let props = properties.read();
        let mut yields = Vec::with_capacity(props.len());

        match binding {
          Iteration::Array(bindings) if bindings.len() == 2 => {
            for (k, v) in props.iter() {
              let elements = vec![Object::String(k.clone()), v.clone()];
              yields.push(self.heap.write().alloc_tuple(elements));
            }
          }
          _ => {
            for k in props.keys() {
              yields.push(Object::String(k.clone()));
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
