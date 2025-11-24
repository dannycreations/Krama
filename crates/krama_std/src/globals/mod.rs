pub mod print;

use krama_core::object::NativeFn;
use krama_core::object::Object;
use rustc_hash::FxHashMap;

pub fn get_globals<'ast>() -> FxHashMap<&'static str, Object<'ast>> {
  let mut globals = FxHashMap::default();
  globals.insert(
    "print",
    Object::NativeFn(NativeFn {
      name: "print",
      callback: print::print,
    }),
  );
  globals
}
