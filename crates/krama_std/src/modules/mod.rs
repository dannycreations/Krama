pub mod assert;
pub mod fs;

use krama_core::object::Object;
use rustc_hash::FxHashMap;

pub fn get_modules<'ast>(
  name: &str,
) -> Option<FxHashMap<&'static str, Object<'ast>>> {
  match name {
    "assert" => Some(assert::get_exports()),
    "fs" => Some(fs::get_exports()),
    _ => None,
  }
}
