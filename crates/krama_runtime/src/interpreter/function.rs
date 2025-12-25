use std::sync::Arc;

use krama_core::{
  FunctionBody, FunctionKind, ObjectKind, Parameter, Type, UserFunction,
};

use super::Interpreter;

impl Interpreter {
  /// Allocates a new UserFunction in the interpreter's arena.
  /// Centralizes function creation to avoid duplication in eval.rs and statement.rs.
  pub fn alloc_user_function(
    &self,
    parameters: Vec<Parameter>,
    body: FunctionBody,
    kind: Option<Type>,
  ) -> ObjectKind {
    let user_fn = Arc::new(UserFunction {
      parameters,
      body,
      kind,
    });
    ObjectKind::Function(FunctionKind::User(user_fn))
  }
}
