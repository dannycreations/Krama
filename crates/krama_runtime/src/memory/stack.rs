use std::sync::Arc;

use ahash::AHashMap;
use krama_core::{Error, ErrorKind, ErrorResult, Object, Scope, Span};
use parking_lot::RwLock;

/// The Call Stack, managing the execution context history.
#[derive(Debug, Clone)]
pub struct Stack {
  /// The stack of scopes representing the call history.
  /// The last element is the current active scope.
  pub frames: Vec<Arc<RwLock<Scope>>>,
}

impl Stack {
  /// Creates a new stack with a global scope.
  pub fn new() -> Self {
    let global_scope =
      Arc::new(RwLock::new(Scope::new(Some("global".into()), None)));
    Self {
      frames: vec![global_scope],
    }
  }

  /// Returns the current active scope.
  pub fn current(&self) -> Arc<RwLock<Scope>> {
    self
      .frames
      .last()
      .expect("Stack underflow: No global scope")
      .clone()
  }

  /// Pushes a new scope onto the stack.
  /// If `parent` is provided, it becomes the parent of the new scope (closure capture).
  /// Otherwise, the current scope becomes the parent (block scope).
  pub fn push(&mut self, name: Arc<str>, parent: Option<Arc<RwLock<Scope>>>) {
    let parent_scope = parent.unwrap_or_else(|| self.current());
    let new_scope =
      Arc::new(RwLock::new(Scope::new(Some(name), Some(parent_scope))));
    self.frames.push(new_scope);
  }

  /// Pops the current scope.
  pub fn pop(&mut self) {
    if self.frames.len() > 1 {
      self.frames.pop();
    } else {
      // Don't pop the global scope
    }
  }

  /// Returns the current depth of the stack (minus global).
  pub fn depth(&self) -> usize {
    self.frames.len() - 1
  }

  /// Searches for a variable starting from the current scope up the chain.
  pub fn get(&self, name: &str) -> Option<Object> {
    self.current().read().get(name)
  }

  /// Sets a variable's value.
  /// Searches up the scope chain to update the nearest binding.
  pub fn set(&mut self, name: &str, value: Object, span: Span) -> ErrorResult {
    let mut current_scope = self.current();

    loop {
      let mut scope = current_scope.write();
      if let Some(binding) = scope.bindings.get_mut(name) {
        if binding.constant {
          return Err(Error::new(
            ErrorKind::TypeError(format!(
              "Cannot assign to constant '{}'",
              name
            )),
            span,
          ));
        }
        binding.value = value;
        return Ok(());
      }

      // Release lock before moving up
      drop(scope);

      let parent = current_scope.read().parent.as_ref().map(Arc::clone);
      if let Some(p) = parent {
        current_scope = p;
      } else {
        break;
      }
    }

    Err(Error::new(
      ErrorKind::ReferenceError(format!("Variable '{}' not found", name)),
      span,
    ))
  }

  /// Defines a variable in the current scope.
  pub fn define(
    &mut self,
    name: Arc<str>,
    value: Object,
    public: bool,
    constant: bool,
  ) {
    self.current().write().set(name, value, public, constant);
  }

  /// Returns all public bindings from the current scope.
  pub fn get_public_bindings(&self) -> AHashMap<Arc<str>, Object> {
    self
      .current()
      .read()
      .bindings
      .iter()
      .filter(|(_, b)| b.public)
      .map(|(name, b)| (name.clone(), b.value.clone()))
      .collect()
  }
}

impl Default for Stack {
  fn default() -> Self {
    Self::new()
  }
}
