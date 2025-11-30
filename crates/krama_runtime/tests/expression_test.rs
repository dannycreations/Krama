use krama_core::{error::ErrorKind, object::Object};
use krama_runtime::{test_eval, test_eval_error};

test_eval!(eval_not_operator, "!true", Object::Boolean(false));

test_eval_error!(eval_negate_non_numeric, "-true", ErrorKind::TypeError(_));

test_eval_error!(
  eval_bitwise_not_non_integer,
  "~true",
  ErrorKind::TypeError(_)
);

test_eval!(
  eval_string_concatenation,
  "\"hello\" + \" world\"",
  Object::String("hello world")
);

test_eval!(
  eval_string_equality,
  "\"a\" == \"a\"",
  Object::Boolean(true)
);

test_eval!(
  eval_string_inequality,
  "\"a\" != \"b\"",
  Object::Boolean(true)
);

test_eval_error!(
  eval_invalid_string_operator,
  "\"a\" * \"b\"",
  ErrorKind::TypeError(_)
);

test_eval!(eval_boolean_equality, "true == true", Object::Boolean(true));

test_eval!(
  eval_boolean_inequality,
  "true != false",
  Object::Boolean(true)
);

test_eval_error!(
  eval_invalid_boolean_operator,
  "true + false",
  ErrorKind::TypeError(_)
);
