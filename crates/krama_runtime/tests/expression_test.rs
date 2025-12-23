use krama_core::{ErrorKind, ObjectKind};
use krama_runtime::{test_eval_err, test_eval_ok};

test_eval_ok!(exp_not_operator, "!true", ObjectKind::Boolean(false));

test_eval_err!(exp_negate_non_numeric, "-true", ErrorKind::TypeError(_));

test_eval_err!(
  exp_bitwise_not_non_integer,
  "~true",
  ErrorKind::TypeError(_)
);

test_eval_ok!(
  exp_string_concatenation,
  "\"hello\" + \" world\"",
  ObjectKind::String("hello world")
);

test_eval_ok!(
  exp_string_equality,
  "\"a\" == \"a\"",
  ObjectKind::Boolean(true)
);

test_eval_ok!(
  exp_string_inequality,
  "\"a\" != \"b\"",
  ObjectKind::Boolean(true)
);

test_eval_err!(
  exp_invalid_string_operator,
  "\"a\" * \"b\"",
  ErrorKind::TypeError(_)
);

test_eval_ok!(
  exp_boolean_equality,
  "true == true",
  ObjectKind::Boolean(true)
);

test_eval_ok!(
  exp_boolean_inequality,
  "true != false",
  ObjectKind::Boolean(true)
);

test_eval_err!(
  exp_invalid_boolean_operator,
  "true + false",
  ErrorKind::TypeError(_)
);
