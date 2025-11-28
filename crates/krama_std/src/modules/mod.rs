#![allow(unreachable_patterns)]

pub mod assert;
pub mod fs;

use krama_core::object::Object;
use rustc_hash::FxHashMap;

#[macro_export]
macro_rules! count_args {
    (@one $($t:tt)*) => { () };
    ($($x:ident),*) => {
        <[()]>::len(&[$($crate::count_args!(@one $x)),*])
    };
}

#[macro_export]
macro_rules! parse_args {
    ($objects:expr, $($arg:ident: $type:pat),*) => {
        const EXPECTED_ARGS: usize = $crate::count_args!($($arg),*);
        if $objects.len() != EXPECTED_ARGS {
            return Err(Error {
                span: Default::default(),
                kind: ErrorKind::ArgumentError(format!(
                    "Expected {} arguments, but got {}",
                    EXPECTED_ARGS,
                    $objects.len()
                )),
            });
        }

        let mut arg_iter = $objects.iter();
        $(
            let $arg = match arg_iter.next() {
                Some($type) => $arg,
                Some(other) => {
                     return Err(Error {
                        span: Default::default(),
                        kind: ErrorKind::ArgumentError(format!(
                            "Expected argument '{}' to be of type '{}', but got '{}'",
                            stringify!($arg),
                            stringify!($type),
                            other.type_name()
                        )),
                    });
                }
                None => unreachable!(),
            };
        )*
    };
}

pub fn get_modules<'ast>(
  name: &str,
) -> Option<FxHashMap<&'static str, Object<'ast>>> {
  match name {
    "assert" => Some(assert::get_exports()),
    "fs" => Some(fs::get_exports()),
    _ => None,
  }
}
