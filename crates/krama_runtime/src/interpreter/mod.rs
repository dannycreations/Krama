mod context;
mod expression;
mod lookup;
mod statement;
mod types;

use std::sync::Arc;

use ahash::AHashMap;
use krama_core::{Function, Object, Span};
use krama_std::GLOBALS;
use parking_lot::RwLock;
pub use types::*;

use crate::{Heap, Stack};

/// The interpreter responsible for executing the AST.
/// Manages execution state, including stack, heap, modules, and local variable resolution.
#[derive(Clone)]
pub struct Interpreter {
  /// Path to the file being executed, if any.
  pub path: Option<String>,
  /// Loaded modules in the current session.
  modules: Arc<RwLock<AHashMap<String, Object>>>,
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
        let function = Object::Function(Function::Native(*native_fn));
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
}
