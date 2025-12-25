mod control;
mod expression;
mod function;
mod statement;
mod types;

use std::sync::Arc;

use ahash::AHashMap;
use krama_core::{
  Error, ErrorKind, ErrorResult, Expression, FunctionKind, ObjectKind,
  ObjectResult, Scope, Span, Statement,
};
use krama_std::GLOBALS;
use parking_lot::RwLock;
pub use types::*;

use crate::{Checker, Heap, Lexer, Parser, Stack};

/// The interpreter responsible for executing the AST.
/// Manages execution state, including stack, heap, modules, and local variable resolution.
#[derive(Clone)]
pub struct Interpreter {
  /// Path to the file being executed, if any.
  pub path: Option<String>,
  /// Loaded modules in the current session.
  pub modules: Arc<RwLock<AHashMap<String, ObjectKind>>>,
  /// The call stack for the current execution thread.
  pub stack: Arc<RwLock<Stack>>,
  /// The heap allocator for complex objects.
  pub heap: Arc<RwLock<Heap>>,
  /// Map of expression spans to their resolved scope distance.
  locals: Arc<RwLock<AHashMap<Span, usize>>>,
}

impl Interpreter {
  /// Creates a new interpreter instance with a global environment.
  pub fn new(path: Option<String>) -> Self {
    let stack = Stack::new();

    // Populate globals
    {
      let current = stack.current();
      let mut scope = current.write();
      scope.bindings.reserve(GLOBALS.len());
      for (name, native_fn) in GLOBALS.iter() {
        let function = ObjectKind::Function(FunctionKind::Native(*native_fn));
        scope.set(Arc::from(*name), function, true, true);
      }
    }

    Self {
      path,
      modules: Arc::new(RwLock::new(AHashMap::default())),
      stack: Arc::new(RwLock::new(stack)),
      heap: Arc::new(RwLock::new(Heap::default())),
      locals: Arc::new(RwLock::new(AHashMap::default())),
    }
  }

  /// Retrieves a variable value from a specific scope distance.
  pub fn get_at(&self, distance: usize, name: &str) -> Option<ObjectKind> {
    let stack = self.stack.read();
    let mut current_scope = stack.current();

    for _ in 0..distance {
      let next = current_scope.read().parent.as_ref().map(Arc::clone)?;
      current_scope = next;
    }

    let scope = current_scope.read();
    scope.get_local(name).map(|b| b.value.clone())
  }

  /// Assigns a value to a variable at a specific scope distance.
  pub fn assign_at(
    &self,
    distance: usize,
    name: &str,
    value: ObjectKind,
    span: Span,
  ) -> ErrorResult {
    let stack = self.stack.read();
    let mut current_scope = stack.current();

    for _ in 0..distance {
      let next = current_scope
        .read()
        .parent
        .as_ref()
        .map(Arc::clone)
        .ok_or_else(|| {
          Error::new(
            ErrorKind::RuntimeError(format!(
              "Invalid scope distance {} for '{}'",
              distance, name
            )),
            span,
          )
        })?;
      current_scope = next;
    }

    let mut scope = current_scope.write();
    if let Some(binding) = scope.bindings.get_mut(name) {
      if binding.constant {
        return Err(Error::new(
          ErrorKind::TypeError(format!("Cannot assign to constant '{}'", name)),
          span,
        ));
      }
      binding.value = value;
      Ok(())
    } else {
      Err(Error::new(
        ErrorKind::ReferenceError(format!(
          "Variable '{}' not found at distance {}",
          name, distance
        )),
        span,
      ))
    }
  }

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
  fn ensure_error_context(&self, mut e: Error, source: &str) -> Error {
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

  /// Returns the resolved scope distance for a given expression.
  pub fn get_resolved_distance(&self, expr: &Expression) -> Option<usize> {
    self.locals.read().get(&expr.span).copied()
  }
}
