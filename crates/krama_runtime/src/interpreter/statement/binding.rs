use std::sync::Arc;

use krama_core::{
  Binding, DestructureBlock, Error, ErrorKind, ErrorResult, Iteration, Object,
  Span,
};

use crate::interpreter::Interpreter;

impl Interpreter {
  pub fn apply_binding(
    &self,
    binding: &Binding,
    value: Object,
    public: bool,
    span: Span,
    constant: bool,
  ) -> ErrorResult {
    match binding {
      Binding::Identifier(name) => {
        self
          .stack
          .write()
          .define(name.clone(), value, public, constant);
      }
      Binding::Destructure(items) => {
        self.apply_destructuring(None, items, value, public, span, constant)?;
      }
      Binding::ModuleAndDestructure { alias, items } => {
        self.apply_destructuring(
          Some(alias),
          items,
          value,
          public,
          span,
          constant,
        )?;
      }
    }
    Ok(())
  }

  pub fn apply_destructuring(
    &self,
    alias: Option<&Arc<str>>,
    items: &[DestructureBlock],
    value: Object,
    public: bool,
    span: Span,
    constant: bool,
  ) -> ErrorResult {
    if let Object::Scope(scope) = &value {
      if let Some(alias_name) = alias {
        self.stack.write().define(
          alias_name.clone(),
          value.clone(),
          public,
          constant,
        );
      }
      for item in items {
        let export_value =
          scope.read().get_local(&item.name).map(|b| b.value.clone());
        if let Some(val) = export_value {
          let name = item.alias.as_ref().unwrap_or(&item.name);
          self
            .stack
            .write()
            .define(name.clone(), val, public, constant);
        } else {
          return Err(Error::new(
            ErrorKind::ReferenceError(format!(
              "'{}' is not exported from module '{}'",
              item.name,
              scope.read().name.as_deref().unwrap_or("<anonymous>")
            )),
            span,
          ));
        }
      }
      Ok(())
    } else {
      Err(Error::new(
        ErrorKind::TypeError(
          "Destructuring can only be done on modules".to_string(),
        ),
        span,
      ))
    }
  }

  pub fn assign_for_binding(
    &self,
    binding: &Iteration,
    value: Object,
    span: Span,
  ) -> ErrorResult {
    match binding {
      Iteration::Identifier(name) => {
        self
          .stack
          .write()
          .define(name.clone(), value.clone(), false, false);
        Ok(())
      }
      Iteration::Array(bindings) => {
        let elements = match &value {
          Object::Array { elements, .. } => elements.read().to_vec(),
          Object::Tuple(elements) => elements.as_ref().to_vec(),
          _ => {
            return Err(Error::new(
              ErrorKind::TypeError(format!(
                "Expected array or tuple for destructuring, found {}",
                value.type_name()
              )),
              span,
            ));
          }
        };

        for (i, binding) in bindings.iter().enumerate() {
          let val = elements.get(i).cloned().unwrap_or(Object::Void);
          self.assign_for_binding(binding, val, span)?;
        }
        Ok(())
      }
    }
  }
}
