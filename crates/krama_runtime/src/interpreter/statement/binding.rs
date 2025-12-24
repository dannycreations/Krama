use krama_core::{
  ConstBinding, Destructure, Error, ErrorKind, ObjectKind, Span,
};

use crate::interpreter::Interpreter;

impl<'ast> Interpreter<'ast> {
  /// Applies a binding (Identifier or Destructuring) to the environment.
  pub(crate) fn apply_binding(
    &self,
    binding: &ConstBinding<'ast>,
    value: ObjectKind<'ast>,
    public: bool,
    span: Span,
  ) -> Result<(), Error<'ast>> {
    match binding {
      ConstBinding::Identifier(name) => {
        self.env_mut(span)?.set(name, value, public, true);
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

  /// Handles destructuring logic for module imports.
  pub(crate) fn apply_destructuring(
    &self,
    alias: Option<&'ast str>,
    items: &[Destructure<'ast>],
    value: ObjectKind<'ast>,
    public: bool,
    span: Span,
  ) -> Result<(), Error<'ast>> {
    if let ObjectKind::Scope(scope) = &value {
      if let Some(alias_name) = alias {
        self
          .env_mut(span)?
          .set(alias_name, value.clone(), public, true);
      }
      for item in items {
        if let Some(export) = scope.get_binding(item.name) {
          let name = item.alias.unwrap_or(item.name);
          self.env_mut(span)?.set(name, export.clone(), public, true);
        } else {
          return Err(Error::new(
            ErrorKind::ReferenceError(format!(
              "'{}' is not exported from module '{}'",
              item.name,
              scope.name.unwrap_or("<anonymous>")
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
}
