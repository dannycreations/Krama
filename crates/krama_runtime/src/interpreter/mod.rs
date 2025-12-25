mod control;
mod expression;
mod function;
mod statement;
mod types;
mod utils;

use std::sync::Arc;

use ahash::AHashMap;
use indexmap::IndexMap;
use krama_core::{Error, ErrorKind, Expression, ObjectKind, Span, Statement};
use parking_lot::{RwLock, RwLockWriteGuard};
pub use types::*;

use crate::{Checker, Environment, Lexer, Parser};

/// The interpreter responsible for executing the AST.
/// Manages execution state, including environment, modules, and local variable resolution.
#[derive(Clone)]
pub struct Interpreter {
  /// The current execution environment.
  pub environment: Arc<RwLock<Environment>>,
  /// Loaded modules in the current session.
  pub modules: Arc<RwLock<IndexMap<String, ObjectKind>>>,
  /// Path to the file being executed, if any.
  pub path: Option<String>,
  /// Map of expression spans to their resolved scope distance.
  locals: Arc<RwLock<AHashMap<Span, usize>>>,
}

impl Interpreter {
  /// Creates a new interpreter instance with a global environment.
  pub fn new(path: Option<String>) -> Self {
    let env = Environment::with_globals();

    Self {
      environment: Arc::new(RwLock::new(env)),
      modules: Arc::new(RwLock::new(IndexMap::default())),
      path,
      locals: Arc::new(RwLock::new(AHashMap::default())),
    }
  }

  /// Creates a new interpreter instance sharing the same arena and modules but with an enclosed environment.
  pub fn new_enclosed(&self) -> Self {
    Self {
      environment: Arc::new(RwLock::new(Environment::new_enclosed(
        self.environment.clone(),
      ))),
      modules: self.modules.clone(),
      path: self.path.clone(),
      locals: self.locals.clone(),
    }
  }

  /// Retrieves a variable value from a specific scope distance.
  pub fn get_at(&self, distance: usize, name: &str) -> Option<ObjectKind> {
    let mut current_env = self.environment.clone();
    for _ in 0..distance {
      let next = current_env.read().outer.clone()?;
      current_env = next;
    }
    let env = current_env.read();
    env.get_local(name)
  }

  /// Assigns a value to a variable at a specific scope distance.
  pub fn assign_at(
    &self,
    distance: usize,
    name: &str,
    value: ObjectKind,
    span: Span,
  ) -> Result<(), Error> {
    let mut current_env = self.environment.clone();
    for _ in 0..distance {
      let next = current_env.read().outer.clone().ok_or_else(|| {
        Error::new(
          ErrorKind::RuntimeError(format!(
            "Invalid scope distance {} for '{}'",
            distance, name
          )),
          span,
        )
      })?;
      current_env = next;
    }
    let mut env = current_env.write();
    if env.is_constant(name) {
      return Err(Error::new(
        ErrorKind::TypeError(format!("Cannot assign to constant '{}'", name)),
        span,
      ));
    }
    env.store.get_mut(name).unwrap().value = value;
    Ok(())
  }

  /// Performs static analysis on the source code without execution.
  pub fn check(&self, source: &str) -> Result<(), Error> {
    self.parse_and_check(source)?;
    Ok(())
  }

  /// Evaluates the source code and returns the result of the last expression.
  pub async fn eval(&self, source: &str) -> Result<ObjectKind, Error> {
    let statements = self.parse_and_check(source)?;
    let result = self.eval_statements(&statements).await?;

    // Use centralized unwrap_return_err to simplify error handling logic.
    let effective_result = result.unwrap_return_err();
    if let ObjectKind::Err(e) = effective_result {
      return Err(self.ensure_error_context(
        Error::new(ErrorKind::RuntimeError(format!("{}", e)), Span::empty()),
        source,
      ));
    }

    // Handle normal return signals.
    Ok(result.unwrap_return().clone())
  }

  /// Parses and runs semantic analysis (checking) on the source.
  pub fn parse_and_check(&self, source: &str) -> Result<Vec<Statement>, Error> {
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
  fn ensure_error_context(&self, e: Error, source: &str) -> Error {
    if e.source.is_none() {
      e.with_context(source, self.path.as_deref().unwrap_or("<unknown>"))
    } else {
      e
    }
  }

  /// Safely borrows the environment mutably, wrapping borrow errors in errors.
  pub fn env_mut(
    &self,
    _span: Span,
  ) -> Result<RwLockWriteGuard<'_, Environment>, Error> {
    Ok(self.environment.write())
  }

  /// Returns the resolved scope distance for a given expression.
  pub fn get_resolved_distance(&self, expr: &Expression) -> Option<usize> {
    self.locals.read().get(&expr.span).copied()
  }
}
