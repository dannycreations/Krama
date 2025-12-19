#![allow(unreachable_patterns)]

macro_rules! count_args {
  ($($x:ident),*) => {
    <[()]>::len(&[$(count_args!(@subst $x)),*])
  };
  (@subst $x:ident) => { () };
}

macro_rules! parse_args {
  ($objects:expr, $fn_name:expr; $($arg:ident: $type:pat),*) => {
    const EXPECTED_ARGS: usize = count_args!($($arg),*);
    if $objects.len() != EXPECTED_ARGS {
      return Err(krama_core::ErrorKind::ArgumentError(format!(
        "{} expected {} arguments, but got {}",
        $fn_name,
        EXPECTED_ARGS,
        $objects.len()
      )));
    }

    let mut arg_iter = $objects.iter();
    $(
      let $arg = match arg_iter.next() {
        Some($type) => $arg,
        Some(other) => {
          return Err(krama_core::ErrorKind::ArgumentError(format!(
            "Expected argument '{}' for function '{}' to be of type '{}', but got '{}'",
            stringify!($arg),
            $fn_name,
            stringify!($type),
            other.type_name()
          )));
        }
        None => unreachable!(),
      };
    )*
  };
}

mod assert;
mod fs;
mod time;
