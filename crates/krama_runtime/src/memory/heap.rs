use std::sync::Arc;

use indexmap::IndexMap;
use krama_core::{Object, Struct, Type};
use parking_lot::RwLock;

/// The Heap Allocator.
/// Responsible for allocating complex objects on the heap wrapped in Arc<RwLock<...>>.
/// This centralizes memory allocation logic and allows for metrics and future GC strategies.
#[derive(Debug, Clone, Default)]
pub struct Heap {
  /// Total number of objects allocated during the heap's lifetime.
  pub allocations: usize,
}

impl Heap {
  /// Allocates a new, empty Object (map-like).
  pub fn alloc_object(
    &mut self,
    properties: IndexMap<Arc<str>, Object>,
    definition: Option<Arc<Struct>>,
    constant: bool,
  ) -> Object {
    self.allocations += 1;
    Object::Object {
      properties: Arc::new(RwLock::new(properties)),
      definition,
      constant,
    }
  }

  /// Allocates a new Tuple.
  pub fn alloc_tuple(&mut self, elements: Vec<Object>) -> Object {
    self.allocations += 1;
    Object::Tuple(elements.into())
  }

  /// Allocates a new Array.
  pub fn alloc_array(
    &mut self,
    elements: Vec<Object>,
    ty: Type,
    constant: bool,
  ) -> Object {
    self.allocations += 1;
    Object::Array {
      elements: Arc::new(RwLock::new(elements)),
      ty,
      constant,
    }
  }

  /// Allocates a raw string (though strings are currently value types in Object,
  /// this is a placeholder if we move to heap-allocated strings).
  pub fn alloc_string(&mut self, s: String) -> Object {
    Object::String(s.into())
  }
}
