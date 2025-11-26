use krama_core::{error::ErrorKind, object::Object};
use krama_internal::{test_eval, test_eval_error};

// Unary Expression Tests
test_eval!(should_eval_not_operator, "!true", Object::Boolean(false));
test_eval_error!(
  should_error_negate_non_numeric,
  "-true",
  ErrorKind::TypeError(_)
);
test_eval_error!(
  should_error_bitwise_not_non_integer,
  "~true",
  ErrorKind::TypeError(_)
);

// Binary Expression Tests
test_eval!(
  should_concatenate_strings,
  "\"hello\" + \" world\"",
  Object::String("hello world")
);
test_eval!(
  should_compare_strings_for_equality,
  "\"a\" == \"a\"",
  Object::Boolean(true)
);
test_eval!(
  should_compare_strings_for_inequality,
  "\"a\" != \"b\"",
  Object::Boolean(true)
);
test_eval_error!(
  should_error_on_invalid_string_op,
  "\"a\" * \"b\"",
  ErrorKind::SyntaxError(_)
);

test_eval!(
  should_compare_booleans_for_equality,
  "true == true",
  Object::Boolean(true)
);
test_eval!(
  should_compare_booleans_for_inequality,
  "true != false",
  Object::Boolean(true)
);
test_eval_error!(
  should_error_on_invalid_bool_op,
  "true + false",
  ErrorKind::SyntaxError(_)
);
