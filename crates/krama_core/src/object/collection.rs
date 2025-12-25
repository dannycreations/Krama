use std::sync::Arc;

use parking_lot::RwLock;

use crate::{ObjectKind, Type};

/// Represents a dynamic array with type safety and interior mutability.
/// Optimized with Arc<RwLock<Vec<T>>> for thread-safe shared access.
#[derive(Debug, Clone)]
pub struct Array {
  pub elements: Arc<RwLock<Vec<ObjectKind>>>,
  pub kind: Type,
  pub constant: bool,
}

impl Array {
  pub fn new(elements: Vec<ObjectKind>, kind: Type, constant: bool) -> Self {
    Self {
      elements: Arc::new(RwLock::new(elements)),
      kind,
      constant,
    }
  }
}
