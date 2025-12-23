use krama_core::{ErrorKind, ObjectKind};
use krama_runtime::{test_eval_err, test_eval_match, test_eval_ok};

test_eval_match!(array_empty, "const a = []; a", ObjectKind::Array { .. });

test_eval_match!(
  array_value,
  "const a: i32[] = [1, 2, 3]; a",
  ObjectKind::Array { .. }
);

test_eval_match!(
  array_fixed,
  "const a: i32[3] = [1, 2, 3]; a",
  ObjectKind::Array { .. }
);

test_eval_match!(
  array_fixed_with_less_elements,
  "const a: i32[3] = [1, 2]; a",
  ObjectKind::Array { .. }
);

test_eval_err!(
  array_fixed_with_more_elements,
  "const a: i32[3] = [1, 2, 3, 4]; a",
  ErrorKind::TypeError(_)
);

test_eval_err!(
  array_with_mixed_types,
  "const a: i32[] = [1, true, 3]; a",
  ErrorKind::TypeError(_)
);

test_eval_ok!(
  array_mutate_value,
  "let a: i32[] = [1]; a[0] = 2; a[0]",
  ObjectKind::Integer(2)
);

test_eval_ok!(
  array_mutate_length,
  "let a: i32[] = [1]; a[1] = 2; a[1]",
  ObjectKind::Integer(2)
);

test_eval_ok!(
  array_fixed_mutate_value,
  "let a: i32[1] = [1]; a[0] = 2; a[0]",
  ObjectKind::Integer(2)
);

test_eval_err!(
  array_fixed_immutability_length,
  "let a: i32[1] = [1]; a[1] = 2; a[1]",
  ErrorKind::TypeError(_)
);

test_eval_err!(
  array_const_immutability,
  "const a: i32[1] = [1]; a[0] = 2; a[0]",
  ErrorKind::TypeError(_)
);

test_eval_err!(
  array_const_update_immutability,
  "const a: i32[] = [1, 2, 3]; a[0]++; a[0]",
  ErrorKind::TypeError(_)
);

test_eval_match!(
  tuple_value,
  "const a = [1, true, \"hello\"]; a",
  ObjectKind::Tuple { .. }
);

test_eval_match!(
  tuple_nested,
  "const a = [1, [true, \"hello\"]]; a",
  ObjectKind::Tuple { .. }
);

test_eval_match!(
  tuple_with_type,
  "const a: [i32, bool] = [1, true]; a",
  ObjectKind::Tuple { .. }
);

test_eval_err!(
  tuple_with_wrong_type,
  "const a: [i32, bool] = [1, 1]; a",
  ErrorKind::TypeError(_)
);

test_eval_err!(
  tuple_with_wrong_length,
  "const a: [i32, bool] = [1]; a",
  ErrorKind::TypeError(_)
);

test_eval_err!(
  tuple_immutability,
  "let a = [1, true]; a[0] = 2; a",
  ErrorKind::TypeError(_)
);

test_eval_err!(
  tuple_update_immutability,
  "let a = [1, true]; a[0]++; a",
  ErrorKind::TypeError(_)
);
