mod behaviour;
mod collection;
mod function;
mod kind;
mod scope;
mod standard;
mod types;

pub use collection::*;
pub use function::*;
pub use kind::*;
pub use scope::*;
pub use standard::*;
pub use types::*;

use crate::ErrorResult;

pub type ObjectResult = ErrorResult<ObjectKind>;
