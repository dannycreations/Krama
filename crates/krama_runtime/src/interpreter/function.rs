use bumpalo::collections::Vec as BumpVec;
use krama_core::{
  FunctionBody, FunctionKind, ObjectKind, Parameter, Type, UserFunction,
};

use super::Interpreter;

impl<'ast> Interpreter<'ast> {
  /// Allocates a new UserFunction in the interpreter's arena.
  /// Centralizes function creation to avoid duplication in eval.rs and statement.rs.
  pub fn alloc_user_function(
    &self,
    parameters: BumpVec<'ast, Parameter<'ast>>,
    body: FunctionBody<'ast>,
    kind: Option<Type<'ast>>,
  ) -> ObjectKind<'ast> {
    let user_fn = self.arena.alloc(UserFunction {
      parameters,
      body,
      kind,
    });
    ObjectKind::Function(FunctionKind::User(user_fn))
  }
}
