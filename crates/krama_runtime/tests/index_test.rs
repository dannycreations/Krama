use krama_core::ObjectKind;
use krama_runtime::test_eval_ok;

test_eval_ok!(
  eval_array_index,
  "const a: i64[3] = [1, 2, 3]; a[0]",
  ObjectKind::Integer(1)
);

test_eval_ok!(
  eval_array_index_negative,
  "const a: i64[3] = [1, 2, 3]; a[-1]",
  ObjectKind::Integer(3)
);

test_eval_ok!(
  eval_array_index_out_of_bounds,
  "const a: i64[3] = [1, 2, 3]; a[3]",
  ObjectKind::Void
);

test_eval_ok!(
  eval_tuple_index,
  "const a = [1, \"hello\", 3]; a[0]",
  ObjectKind::Integer(1)
);

test_eval_ok!(
  eval_tuple_index_negative,
  "const a = [1, \"hello\", 3]; a[-1]",
  ObjectKind::Integer(3)
);

test_eval_ok!(
  eval_tuple_index_out_of_bounds,
  "const a = [1, \"hello\", 3]; a[3]",
  ObjectKind::Void
);

test_eval_ok!(
  eval_string_index,
  "const a = \"hello\"; a[0]",
  ObjectKind::String("h")
);

test_eval_ok!(
  eval_string_index_negative,
  "const a = \"hello\"; a[-1]",
  ObjectKind::String("o")
);

test_eval_ok!(
  eval_string_index_out_of_bounds,
  "const a = \"hello\"; a[5]",
  ObjectKind::Void
);
