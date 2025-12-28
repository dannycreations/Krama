use std::sync::Arc;

use linkme::distributed_slice;
use parking_lot::RwLock;

use crate::{ErrorResult, Type};

mod function;
mod kind;
mod scope;
mod types;

pub use function::*;
pub use kind::*;
pub use scope::*;
pub use types::*;

/// Result type for operations returning an Object.
pub type ObjectResult = ErrorResult<Object>;

/// Registration for globally available functions.
pub struct StandardGlobal {
  pub name: &'static str,
  pub callback: NativeFnCb,
}

#[distributed_slice]
pub static STANDARD_GLOBALS: [StandardGlobal];

/// Registration for built-in modules.
pub struct StandardModule {
  pub name: &'static str,
  pub callback: NativeFnCb,
  pub module: &'static str,
}

#[distributed_slice]
pub static STANDARD_MODULES: [StandardModule];

/// Registration for built-in properties on specific types.
pub struct StandardProperty {
  pub name: &'static str,
  pub callback: PropertyFnCb,
  pub types: &'static [&'static str],
}

#[distributed_slice]
pub static STANDARD_PROPERTIES: [StandardProperty];

/// Represents a dynamic array with type safety and interior mutability.
#[derive(Debug, Clone)]
pub struct Array {
  pub elements: Arc<RwLock<Vec<Object>>>,
  pub ty: Type,
  pub constant: bool,
}

impl Array {
  pub fn new(elements: Vec<Object>, ty: Type, constant: bool) -> Self {
    Self {
      elements: Arc::new(RwLock::new(elements)),
      ty,
      constant,
    }
  }
}
