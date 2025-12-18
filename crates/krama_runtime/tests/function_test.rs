use krama_core::{ErrorKind, Object};
use krama_runtime::{test_eval_err, test_eval_ok};

test_eval_ok!(
  eval_native_function_call,
  "print(\"hello world\")",
  Object::Void
);

test_eval_ok!(
  eval_let_bound_function_call,
  "let a = fn(x) { x }; a(5)",
  Object::Integer(5)
);

test_eval_ok!(
  eval_const_bound_function_call,
  "const a = fn(x) { x }; a(5)",
  Object::Integer(5)
);

test_eval_ok!(
  eval_arrow_function_call,
  "let a = (x) => x; a(5)",
  Object::Integer(5)
);

test_eval_err!(
  eval_arrow_function_call_with_block,
  "let a = (x) => { x }; a(5)",
  ErrorKind::SyntaxError(_)
);

test_eval_err!(
  eval_classic_function_call_with_arrow,
  "let a = fn(x) => x; a(5)",
  ErrorKind::SyntaxError(_)
);

test_eval_ok!(
  eval_function_with_statement,
  "fn a(x, y) { x + y }; a(5, 5)",
  Object::Integer(10)
);

test_eval_ok!(
  eval_function_with_return_statement,
  "fn a(x, y) { return x + y }; a(5, 5)",
  Object::Integer(10)
);

test_eval_ok!(
  eval_function_with_default_argument,
  "fn a(x = 0) { x }; a()",
  Object::Integer(0)
);

test_eval_ok!(
  eval_function_with_passed_argument,
  "fn a(x = 10) { x }; a(1)",
  Object::Integer(1)
);

test_eval_ok!(
  eval_arrow_function_with_default_argument,
  "const a = (x = 0) => x; a()",
  Object::Integer(0)
);

test_eval_ok!(
  eval_arrow_function_with_passed_argument,
  "const a = (x = 0) => x; a(1)",
  Object::Integer(1)
);

test_eval_ok!(
  eval_arrow_function_with_optional_argument,
  "const a = (x, y = 0) => x + 0; a(1)",
  Object::Integer(1)
);

test_eval_err!(
  eval_error_on_missing_required_argument,
  "fn a(x, y) { x }; a(1)",
  ErrorKind::TypeError(_)
);

test_eval_err!(
  eval_error_on_missing_argument_after_optional,
  "fn a(x = 0, y) { x }; a()",
  ErrorKind::TypeError(_)
);
