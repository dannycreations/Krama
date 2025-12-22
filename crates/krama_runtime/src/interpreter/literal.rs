use krama_core::{Error, LiteralKind, ObjectKind};

use crate::Interpreter;

impl<'ast> Interpreter<'ast> {
  /// Evaluates a literal into an Object.
  /// Marked with #[inline] to encourage the compiler to embed this small function.
  #[inline]
  pub fn eval_literal(
    &self,
    literal: LiteralKind<'ast>,
  ) -> Result<ObjectKind<'ast>, Error<'ast>> {
    Ok(match literal {
      LiteralKind::Integer(i) => ObjectKind::Integer(i),
      LiteralKind::Float(f) => ObjectKind::Float(f),
      LiteralKind::String(s) => ObjectKind::String(s),
      LiteralKind::Boolean(b) => ObjectKind::Boolean(b),
      LiteralKind::Null => ObjectKind::Null,
    })
  }
}
