use krama_core::{Error, ErrorKind, ErrorResult, ObjectKind, Span};

use crate::interpreter::Interpreter;

impl Interpreter {
  /// Verifies if a member is accessible based on its visibility and the current execution context.
  pub fn ensure_accessible(
    &self,
    public: bool,
    member_name: &str,
    struct_name: &str,
    span: Span,
  ) -> ErrorResult {
    if public {
      return Ok(());
    }

    // Check if the current scope is within the same struct definition.
    let stack = self.stack.read();
    let current_struct = stack.get("__current_struct__");
    let allowed = if let Some(ObjectKind::String(name)) = current_struct {
      name == struct_name
    } else {
      false
    };

    if !allowed {
      return Err(Error::new(
        ErrorKind::TypeError(format!("Member '{}' is private", member_name)),
        span,
      ));
    }
    Ok(())
  }
}
