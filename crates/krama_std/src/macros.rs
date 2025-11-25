#[macro_export]
macro_rules! count_args {
    (@one $($t:tt)*) => { () };
    ($($x:ident),*) => {
        <[()]>::len(&[$(count_args!(@one $x)),*])
    };
}

#[macro_export]
macro_rules! parse_args {
    ($objects:expr, $($arg:ident: $type:pat),*) => {
        const EXPECTED_ARGS: usize = $crate::count_args!($($arg),*);
        if $objects.len() != EXPECTED_ARGS {
            return Err(Error {
                span: Default::default(),
                kind: ErrorKind::WrongNumberOfArguments {
                    expected: EXPECTED_ARGS,
                    got: $objects.len(),
                },
            });
        }

        let mut arg_iter = $objects.iter();
        $(
            let $arg = match arg_iter.next().unwrap() {
                $type => $arg,
                other => {
                    return Err(Error {
                        span: Default::default(),
                        kind: ErrorKind::TypeMismatch(format!(
                            "Expected argument '{}' to be of type '{}', but got {}",
                            stringify!($arg),
                            stringify!($type),
                            other.to_string()
                        )),
                    });
                }
            };
        )*
    };
}
