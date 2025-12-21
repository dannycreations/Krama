use krama_core::Object;
use krama_runtime::test_eval_ok;

test_eval_ok!(
  eval_get_string_length,
  "\"hello\".length",
  Object::Integer(5)
);

test_eval_ok!(
  eval_get_array_length,
  "const a: i32[] = [1, 2, 3]; a.length",
  Object::Integer(3)
);

test_eval_ok!(
  eval_get_tuple_length,
  "[1, \"hello\", 3].length",
  Object::Integer(3)
);
