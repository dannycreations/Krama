mod control;
mod expression;
mod function;
mod statement;
mod types;
mod utils;

use std::cell::{RefCell, RefMut};

use ahash::AHashMap;
use bumpalo::Bump;
use indexmap::IndexMap;
use krama_core::{Error, ErrorKind, Expression, ObjectKind, Program, Span};
pub use types::*;

use crate::{Checker, Environment, Lexer, Parser};

/// The interpreter responsible for executing the AST.
/// Manages execution state, including environment, modules, and local variable resolution.
#[derive(Clone)]
pub struct Interpreter<'ast> {
  /// The current execution environment.
  pub environment: &'ast RefCell<Environment<'ast>>,
  /// Loaded modules in the current session.
  pub modules: &'ast RefCell<IndexMap<&'ast str, ObjectKind<'ast>>>,
  /// Memory arena for AST and runtime object allocations.
  pub arena: &'ast Bump,
  /// Path to the file being executed, if any.
  pub path: Option<&'ast str>,
  /// Map of expression spans to their resolved scope distance.
  locals: RefCell<AHashMap<Span, usize>>,
}

impl<'ast> Interpreter<'ast> {
  /// Creates a new interpreter instance with a global environment.
  pub fn new(arena: &'ast Bump, path: Option<&'ast str>) -> Self {
    let env = Environment::with_globals();

    Self {
      environment: arena.alloc(RefCell::new(env)),
      modules: arena.alloc(RefCell::new(IndexMap::default())),
      arena,
      path,
      locals: RefCell::new(AHashMap::default()),
    }
  }

  /// Creates a new interpreter instance sharing the same arena and modules but with an enclosed environment.
  pub fn new_enclosed(&self) -> Self {
    Self {
      environment: self
        .arena
        .alloc(RefCell::new(Environment::new_enclosed(self.environment))),
      modules: self.modules,
      arena: self.arena,
      path: self.path,
      locals: self.locals.clone(),
    }
  }

  /// Retrieves a variable value from a specific scope distance.
  pub fn get_at(
    &self,
    distance: usize,
    name: &str,
  ) -> Option<ObjectKind<'ast>> {
    let mut env_cell = self.environment;
    for _ in 0..distance {
      let next = env_cell.borrow().outer?;
      env_cell = next;
    }
    env_cell.borrow().get_local(name)
  }

  /// Assigns a value to a variable at a specific scope distance.
  pub fn assign_at(
    &self,
    distance: usize,
    name: &'ast str,
    value: ObjectKind<'ast>,
    span: Span,
  ) -> Result<(), Error<'ast>> {
    let mut env_cell = self.environment;
    for _ in 0..distance {
      let next = env_cell.borrow().outer.ok_or_else(|| {
        Error::new(
          ErrorKind::RuntimeError(format!(
            "Invalid scope distance {} for '{}'",
            distance, name
          )),
          span,
        )
      })?;
      env_cell = next;
    }
    let mut env = env_cell.borrow_mut();
    if env.is_constant(name) {
      return Err(Error::new(
        ErrorKind::TypeError(format!("Cannot assign to constant '{}'", name)),
        span,
      ));
    }
    env.store.get_mut(name).unwrap().value = value;
    Ok(())
  }

  /// Allocates a string into the interpreter's arena.
  pub fn alloc_str(&self, s: &str) -> &'ast str {
    self.arena.alloc_str(s)
  }

  /// Performs static analysis on the source code without execution.
  pub fn check(&self, source: &'ast str) -> Result<(), Error<'ast>> {
    self.parse_and_check(source)?;
    Ok(())
  }

  /// Evaluates the source code and returns the result of the last expression.
  pub async fn eval(
    &self,
    source: &'ast str,
  ) -> Result<ObjectKind<'ast>, Error<'ast>> {
    let program = self.parse_and_check(source)?;
    let result = self.eval_statements(&program.statements).await?;

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
  pub fn parse_and_check(
    &self,
    source: &'ast str,
  ) -> Result<Program<'ast>, Error<'ast>> {
    let lexer = Lexer::new(source, self.path);
    let mut parser = Parser::new(lexer, self.arena);
    let program = parser
      .parse()
      .map_err(|e| self.ensure_error_context(e, source))?;

    let mut checker = Checker::new();
    let locals = checker
      .check(&program)
      .map_err(|e| self.ensure_error_context(e, source))?;

    *self.locals.borrow_mut() = locals.into_iter().collect();
    Ok(program)
  }

  /// Ensures an error has source and file context for better diagnostics.
  fn ensure_error_context(
    &self,
    e: Error<'ast>,
    source: &'ast str,
  ) -> Error<'ast> {
    if e.source.is_none() {
      e.with_context(source, self.path.unwrap_or("<unknown>"))
    } else {
      e
    }
  }

  /// Safely borrows the environment mutably, wrapping borrow errors in errors.
  pub fn env_mut(
    &self,
    span: Span,
  ) -> Result<RefMut<'_, Environment<'ast>>, Error<'ast>> {
    self
      .environment
      .try_borrow_mut()
      .map_err(|e| Error::new(ErrorKind::RuntimeError(e.to_string()), span))
  }

  /// Returns the resolved scope distance for a given expression.
  pub fn get_resolved_distance(
    &self,
    expr: &Expression<'ast>,
  ) -> Option<usize> {
    self.locals.borrow().get(&expr.span).copied()
  }
}
