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

/// # Safety
///
/// This function is unsafe because it transmutes the lifetime of an `Object`.
/// It is only safe to call this function when transmuting an `Object<'static>`
/// to an `Object<'ast>`, where `'ast` is a lifetime tied to a Bump arena.
/// This is sound because `'static` outlives any other lifetime, ensuring that
/// any borrowed data within the object remains valid for the duration of `'ast`.
pub unsafe fn transmute_static_object_to_ast<'ast>(
  obj: Object<'static>,
) -> Object<'ast> {
  std::mem::transmute(obj)
}
