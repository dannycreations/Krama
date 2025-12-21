use krama_core::Object;
use krama_runtime::test_eval_ok;

test_eval_ok!(
  eval_array_index,
  "const a: i64[3] = [1, 2, 3]; a[0]",
  Object::Integer(1)
);

test_eval_ok!(
  eval_array_index_negative,
  "const a: i64[3] = [1, 2, 3]; a[-1]",
  Object::Integer(3)
);

test_eval_ok!(
  eval_array_index_out_of_bounds,
  "const a: i64[3] = [1, 2, 3]; a[3]",
  Object::Void
);

test_eval_ok!(
  eval_tuple_index,
  "const a = [1, \"hello\", 3]; a[0]",
  Object::Integer(1)
);

test_eval_ok!(
  eval_tuple_index_negative,
  "const a = [1, \"hello\", 3]; a[-1]",
  Object::Integer(3)
);

test_eval_ok!(
  eval_tuple_index_out_of_bounds,
  "const a = [1, \"hello\", 3]; a[3]",
  Object::Void
);

test_eval_ok!(
  eval_string_index,
  "const a = \"hello\"; a[0]",
  Object::String("h")
);

test_eval_ok!(
  eval_string_index_negative,
  "const a = \"hello\"; a[-1]",
  Object::String("o")
);

test_eval_ok!(
  eval_string_index_out_of_bounds,
  "const a = \"hello\"; a[5]",
  Object::Void
);
