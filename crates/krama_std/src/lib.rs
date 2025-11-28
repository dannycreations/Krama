pub mod globals;
pub mod modules;
pub mod props;

use krama_core::object::{Function, NativeFunction, NativeFunctionCb, Object};
use rustc_hash::FxHashMap;

pub(crate) fn build_native_functions<'ast>(
  fns: &[(&'static str, NativeFunctionCb<'ast>)],
) -> FxHashMap<&'static str, Object<'ast>> {
  fns
    .iter()
    .map(|(name, callback)| {
      (
        *name,
        Object::Function(Function::Native(NativeFunction {
          name,
          callback: *callback,
        })),
      )
    })
    .collect()
}
