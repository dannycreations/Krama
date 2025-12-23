use krama_core::ObjectKind;
use krama_runtime::test_eval_ok;

test_eval_ok!(
  prop_get_string_length,
  "\"hello\".length",
  ObjectKind::Integer(5)
);

test_eval_ok!(
  prop_get_array_length,
  "const a: i32[] = [1, 2, 3]; a.length",
  ObjectKind::Integer(3)
);

test_eval_ok!(
  prop_get_tuple_length,
  "[1, \"hello\", 3].length",
  ObjectKind::Integer(3)
);
