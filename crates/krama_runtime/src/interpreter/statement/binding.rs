use std::sync::Arc;

use krama_core::{
  ConstBinding, Destructure, Error, ErrorKind, ErrorResult, ForBinding,
  ObjectKind, Span,
};

use crate::interpreter::Interpreter;

impl Interpreter {
  pub fn apply_binding(
    &self,
    binding: &ConstBinding,
    value: ObjectKind,
    public: bool,
    span: Span,
  ) -> ErrorResult {
    match binding {
      ConstBinding::Identifier(name) => {
        self.stack.write().define(name.clone(), value, public, true);
      }
      ConstBinding::Destructure(items) => {
        self.apply_destructuring(None, items, value, public, span)?;
      }
      ConstBinding::ModuleAndDestructure { alias, items } => {
        self.apply_destructuring(Some(alias), items, value, public, span)?;
      }
    }
    Ok(())
  }

  pub fn apply_destructuring(
    &self,
    alias: Option<&Arc<str>>,
    items: &[Destructure],
    value: ObjectKind,
    public: bool,
    span: Span,
  ) -> ErrorResult {
    if let ObjectKind::Scope(scope) = &value {
      if let Some(alias_name) = alias {
        self.stack.write().define(
          alias_name.clone(),
          value.clone(),
          public,
          true,
        );
      }
      for item in items {
        let export_value =
          scope.read().get_local(&item.name).map(|b| b.value.clone());
        if let Some(val) = export_value {
          let name = item.alias.as_ref().unwrap_or(&item.name);
          self.stack.write().define(name.clone(), val, public, true);
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
    binding: &ForBinding,
    value: ObjectKind,
    span: Span,
  ) -> ErrorResult {
    match binding {
      ForBinding::Identifier(name) => {
        self
          .stack
          .write()
          .define(name.clone(), value.clone(), false, false);
        Ok(())
      }
      ForBinding::Array(bindings) => {
        let elements = match &value {
          ObjectKind::Array { elements, .. } => elements.read().to_vec(),
          ObjectKind::Tuple(elements) => elements.as_ref().to_vec(),
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
          let val = elements.get(i).cloned().unwrap_or(ObjectKind::Void);
          self.assign_for_binding(binding, val, span)?;
        }
        Ok(())
      }
    }
  }
}
