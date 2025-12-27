use std::sync::Arc;

use krama_core::{
  Error, ErrorKind, ErrorResult, ObjectKind, ObjectResult, Scope, Span,
  Statement,
};
use parking_lot::RwLock;

use super::Interpreter;
use crate::{Checker, Lexer, Parser};

impl Interpreter {
  /// Performs static analysis on the source code without execution.
  pub fn check(&self, source: &str) -> ErrorResult {
    self.parse_and_check(source).map(|_| ())
  }

  /// Evaluates the source code and returns the result of the last expression.
  pub async fn eval(&self, source: &str) -> ObjectResult {
    let statements = self.parse_and_check(source)?;
    let result = self.eval_statements(&statements).await?;

    // Handle both explicit Return(Err) and direct Err results.
    let effective_result = result.unwrap_return_err();
    if let ObjectKind::Err(e) = effective_result {
      return Err(self.ensure_error_context(
        ErrorKind::RuntimeError(e.to_string()).at(Span::empty()),
        source,
      ));
    }

    Ok(result.unwrap_return().clone())
  }

  /// Parses and runs semantic analysis (checking) on the source.
  pub fn parse_and_check(&self, source: &str) -> ErrorResult<Vec<Statement>> {
    let lexer = Lexer::new(source, self.path.clone());
    let mut parser = Parser::new(lexer);
    let statements = parser
      .parse()
      .map_err(|e| self.ensure_error_context(e, source))?;

    let mut checker = Checker::new();
    let locals = checker
      .check(&statements)
      .map_err(|e| self.ensure_error_context(e, source))?;

    *self.locals.write() = locals.into_iter().collect();
    Ok(statements)
  }

  /// Ensures an error has source and file context for better diagnostics.
  /// If the error already has source context, it is returned as-is.
  pub(crate) fn ensure_error_context(
    &self,
    mut e: Error,
    source: &str,
  ) -> Error {
    if e.source.is_none() {
      e.source = Some(source.to_string());
      e.file = self.path.clone().or_else(|| Some("<unknown>".to_string()));
    }
    e
  }

  /// Returns the current scope for mutation.
  /// Caller is responsible for locking.
  pub fn current_scope(&self) -> Arc<RwLock<Scope>> {
    self.stack.read().current()
  }
}
