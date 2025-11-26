pub mod globals;
mod macros;
pub mod modules;
pub mod props;

use krama_core::object::{Function, NativeFn, NativeFnCallback, Object};
use rustc_hash::FxHashMap;

pub(crate) fn build_native_functions<'ast>(
  fns: &[(&'static str, NativeFnCallback<'ast>)],
) -> FxHashMap<&'static str, Object<'ast>> {
  fns
    .iter()
    .map(|(name, callback)| {
      (
        *name,
        Object::Function(Function::Native(NativeFn {
          name,
          callback: *callback,
        })),
      )
    })
    .collect()
}
