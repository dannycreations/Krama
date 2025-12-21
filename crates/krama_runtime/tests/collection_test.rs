use krama_core::{ErrorKind, Object};
use krama_runtime::{test_eval_err, test_eval_match, test_eval_ok};

test_eval_match!(eval_array, "const a = []; a", Object::Array { .. });

test_eval_match!(
  eval_array_value,
  "const a: i32[] = [1, 2, 3]; a",
  Object::Array { .. }
);

test_eval_match!(
  eval_array_fixed,
  "const a: i32[3] = [1, 2, 3]; a",
  Object::Array { .. }
);

test_eval_match!(
  eval_array_fixed_with_less_elements,
  "const a: i32[3] = [1, 2]; a",
  Object::Array { .. }
);

test_eval_err!(
  eval_array_fixed_with_more_elements,
  "const a: i32[3] = [1, 2, 3, 4]; a",
  ErrorKind::TypeError(_)
);

test_eval_err!(
  eval_array_with_mixed_types,
  "const a: i32[] = [1, true, 3]; a",
  ErrorKind::TypeError(_)
);

test_eval_ok!(
  eval_array_mutate_value,
  "let a: i32[] = [1]; a[0] = 2; a[0]",
  Object::Integer(2)
);

test_eval_ok!(
  eval_array_mutate_length,
  "let a: i32[] = [1]; a[1] = 2; a[1]",
  Object::Integer(2)
);

test_eval_ok!(
  eval_array_fixed_mutate_value,
  "let a: i32[1] = [1]; a[0] = 2; a[0]",
  Object::Integer(2)
);

test_eval_err!(
  eval_array_fixed_immutability_length,
  "let a: i32[1] = [1]; a[1] = 2; a[1]",
  ErrorKind::TypeError(_)
);

test_eval_err!(
  eval_const_array_immutability,
  "const a: i32[1] = [1]; a[0] = 2; a[0]",
  ErrorKind::TypeError(_)
);

test_eval_err!(
  eval_const_array_update_immutability,
  "const a: i32[] = [1, 2, 3]; a[0]++; a[0]",
  ErrorKind::TypeError(_)
);

test_eval_match!(
  eval_tuple,
  "const a = [1, true, \"hello\"]; a",
  Object::Tuple { .. }
);

test_eval_match!(
  eval_tuple_nested,
  "const a = [1, [true, \"hello\"]]; a",
  Object::Tuple { .. }
);

test_eval_match!(
  eval_tuple_with_type,
  "const a: [i32, bool] = [1, true]; a",
  Object::Tuple { .. }
);

test_eval_err!(
  eval_tuple_with_wrong_type,
  "const a: [i32, bool] = [1, 1]; a",
  ErrorKind::TypeError(_)
);

test_eval_err!(
  eval_tuple_with_wrong_length,
  "const a: [i32, bool] = [1]; a",
  ErrorKind::TypeError(_)
);

test_eval_err!(
  eval_tuple_immutability,
  "let a = [1, true]; a[0] = 2; a",
  ErrorKind::TypeError(_)
);

test_eval_err!(
  eval_tuple_update_immutability,
  "let a = [1, true]; a[0]++; a",
  ErrorKind::TypeError(_)
);
