use krama_core::{ErrorKind, ObjectKind};
use krama_runtime::{test_eval_err, test_eval_ok};

test_eval_ok!(function_call, "print(\"hello world\")", ObjectKind::Void);

test_eval_ok!(
  function_let_bound,
  "let a = fn(x) { x }; a = a(5)",
  ObjectKind::Integer(5)
);

test_eval_ok!(
  function_const_bound,
  "const a = fn(x) { x }; a(5)",
  ObjectKind::Integer(5)
);

test_eval_ok!(
  function_arrow,
  "const a = (x) => x; a(5)",
  ObjectKind::Integer(5)
);

test_eval_err!(
  function_arrow_block,
  "const a = (x) => { x }; a(5)",
  ErrorKind::SyntaxError(_)
);

test_eval_err!(
  function_arrow_classic,
  "const a = fn(x) => x; a(5)",
  ErrorKind::SyntaxError(_)
);

test_eval_ok!(
  function_with_statement,
  "fn a(x, y) { x + y }; a(5, 5)",
  ObjectKind::Integer(10)
);

test_eval_ok!(
  function_with_return_statement,
  "fn a(x, y) { return x + y }; a(5, 5)",
  ObjectKind::Integer(10)
);

test_eval_ok!(
  function_with_default_argument,
  "fn a(x = 0) { x }; a()",
  ObjectKind::Integer(0)
);

test_eval_ok!(
  function_with_passed_argument,
  "fn a(x = 10) { x }; a(1)",
  ObjectKind::Integer(1)
);

test_eval_ok!(
  function_arrow_with_default_argument,
  "const a = (x = 0) => x; a()",
  ObjectKind::Integer(0)
);

test_eval_ok!(
  function_arrow_with_passed_argument,
  "const a = (x = 0) => x; a(1)",
  ObjectKind::Integer(1)
);

test_eval_ok!(
  function_arrow_with_optional_argument,
  "const a = (x, y = 0) => x + 0; a(1)",
  ObjectKind::Integer(1)
);

test_eval_err!(
  function_missing_required_argument,
  "fn a(x, y) { x }; a(1)",
  ErrorKind::TypeError(_)
);

test_eval_err!(
  function_missing_argument_after_optional,
  "fn a(x = 0, y) { x }; a()",
  ErrorKind::TypeError(_)
);
