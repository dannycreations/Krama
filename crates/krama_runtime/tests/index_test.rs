use krama_core::ObjectKind;
use krama_runtime::test_eval_ok;

test_eval_ok!(
  index_array,
  "const a: i64[3] = [1, 2, 3]; a[0]",
  ObjectKind::Integer(1)
);

test_eval_ok!(
  index_array_negative,
  "const a: i64[3] = [1, 2, 3]; a[-1]",
  ObjectKind::Integer(3)
);

test_eval_ok!(
  index_array_out_of_bounds,
  "const a: i64[3] = [1, 2, 3]; a[3]",
  ObjectKind::Void
);

test_eval_ok!(
  index_tuple,
  "const a = [1, \"hello\", 3]; a[0]",
  ObjectKind::Integer(1)
);

test_eval_ok!(
  index_tuple_negative,
  "const a = [1, \"hello\", 3]; a[-1]",
  ObjectKind::Integer(3)
);

test_eval_ok!(
  index_tuple_out_of_bounds,
  "const a = [1, \"hello\", 3]; a[3]",
  ObjectKind::Void
);

test_eval_ok!(
  index_string,
  "const a = \"hello\"; a[0]",
  ObjectKind::String("h")
);

test_eval_ok!(
  index_string_negative,
  "const a = \"hello\"; a[-1]",
  ObjectKind::String("o")
);

test_eval_ok!(
  index_string_out_of_bounds,
  "const a = \"hello\"; a[5]",
  ObjectKind::Void
);
