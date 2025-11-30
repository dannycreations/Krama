use krama_core::{error::ErrorKind, object::Object};
use krama_runtime::{test_eval, test_eval_error};

test_eval!(
  eval_native_function_call,
  r#"print("hello world")"#,
  Object::Void
);

test_eval!(
  eval_user_defined_function_call,
  r#"fn a(x) { x }; a(5)"#,
  Object::Integer(5)
);

test_eval!(
  eval_let_bound_function_call,
  r#"let a = (x) { x }; a(5)"#,
  Object::Integer(5)
);

test_eval!(
  eval_const_bound_function_call,
  r#"const a = (x) { x }; a(5)"#,
  Object::Integer(5)
);

test_eval!(
  eval_arrow_function_call,
  r#"let a = (x) => x; a(5)"#,
  Object::Integer(5)
);

test_eval!(
  eval_function_with_statement,
  r#"fn a(x, y) { x + y }; a(5, 5)"#,
  Object::Integer(10)
);

test_eval!(
  eval_function_with_return_statement,
  r#"fn a(x, y) { return x + y }; a(5, 5)"#,
  Object::Integer(10)
);

test_eval!(
  eval_function_with_default_argument,
  r#"fn a(x = 0) { x }; a()"#,
  Object::Integer(0)
);

test_eval!(
  eval_function_with_passed_argument,
  r#"fn a(x = 10) { x }; a(1)"#,
  Object::Integer(1)
);

test_eval!(
  eval_arrow_function_with_default_argument,
  r#"const a = (x = 0) => x; a()"#,
  Object::Integer(0)
);

test_eval!(
  eval_arrow_function_with_passed_argument,
  r#"const a = (x = 0) => x; a(1)"#,
  Object::Integer(1)
);

test_eval!(
  eval_arrow_function_with_optional_argument,
  r#"const a = (x, y = 0) => x + 0; a(1)"#,
  Object::Integer(1)
);

test_eval_error!(
  eval_error_on_missing_required_argument,
  r#"fn a(x, y) { x }; a(1)"#,
  ErrorKind::TypeError(_)
);

test_eval_error!(
  eval_error_on_missing_argument_after_optional,
  r#"fn a(x = 0, y) { x }; a()"#,
  ErrorKind::TypeError(_)
);
