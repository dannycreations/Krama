use krama_core::{error::ErrorKind, object::Object};
use krama_internal::{test_eval, test_eval_error};

test_eval!(
  should_call_native_fn,
  r#"print("hello world")"#,
  Object::Void
);

test_eval!(
  should_call_user_defined_fn,
  r#"fn a(x) { x }; a(5)"#,
  Object::Integer(5)
);

test_eval!(
  should_call_let_bound_fn,
  r#"let a = (x) { x }; a(5)"#,
  Object::Integer(5)
);

test_eval!(
  should_call_const_bound_fn,
  r#"const a = (x) { x }; a(5)"#,
  Object::Integer(5)
);

test_eval!(
  should_call_arrow_fn,
  r#"let a = (x) => x; a(5)"#,
  Object::Integer(5)
);

test_eval!(
  should_handle_stmt_in_fn,
  r#"fn a(x, y) { x + y }; a(5, 5)"#,
  Object::Integer(10)
);

test_eval!(
  should_handle_return_stmt_in_fn,
  r#"fn a(x, y) { return x + y }; a(5, 5)"#,
  Object::Integer(10)
);

test_eval!(
  should_use_default_argument,
  r#"fn a(x = 0) { x }; a()"#,
  Object::Integer(0)
);

test_eval!(
  should_use_passed_argument,
  r#"fn a(x = 10) { x }; a(1)"#,
  Object::Integer(1)
);

test_eval!(
  should_use_default_argument_in_arrow_fn,
  r#"const a = (x = 0) => x; a()"#,
  Object::Integer(0)
);

test_eval!(
  should_use_passed_argument_in_arrow_fn,
  r#"const a = (x = 0) => x; a(1)"#,
  Object::Integer(1)
);

test_eval!(
  should_use_optional_argument_in_arrow_fn,
  r#"const a = (x, y = 0) => x + 0; a(1)"#,
  Object::Integer(1)
);

test_eval_error!(
  should_panic_if_required_argument_is_missing,
  r#"fn a(x, y) { x }; a(1)"#,
  ErrorKind::TypeError(_)
);

test_eval_error!(
  should_panic_if_required_after_optional_is_missing,
  r#"fn a(x = 0, y) { x }; a()"#,
  ErrorKind::TypeError(_)
);
