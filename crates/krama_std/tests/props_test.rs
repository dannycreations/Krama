use krama_core::object::Object;
use krama_internal::test_eval;

test_eval!(
  should_get_string_length,
  "\"hello\".length",
  Object::Integer(5)
);

test_eval!(
  should_get_array_length,
  "const a: i64[] = [1, 2, 3]; a.length",
  Object::Integer(3)
);

test_eval!(
  should_get_tuple_length,
  "[1, \"hello\", 3].length",
  Object::Integer(3)
);
