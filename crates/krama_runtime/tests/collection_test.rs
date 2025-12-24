use krama_core::{ErrorKind, ObjectKind};
use krama_runtime::{test_eval_err, test_eval_match, test_eval_ok};

// --- Array Creation & Typing ---

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

// --- Array Mutation ---

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

// --- Nested Collection Mutation ---

test_eval_ok!(
  array_nested_mutation,
  r#"
    let a: i32[][] = [[1, 2], [3, 4]]
    a[0][1] = 10
    a[0][1]
  "#,
  ObjectKind::Integer(10)
);

test_eval_ok!(
  array_shared_reference_mutation,
  r#"
    let a: i32[] = [1, 2]
    let b: i32[][] = [a, a]
    b[0][0] = 10
    b[1][0]
  "#,
  ObjectKind::Integer(10)
);

// --- Array Immutability ---

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

// --- Array Indexing ---

test_eval_ok!(
  array_index_access,
  "const a: i64[3] = [1, 2, 3]; a[0]",
  ObjectKind::Integer(1)
);

test_eval_ok!(
  array_index_negative,
  "const a: i64[3] = [1, 2, 3]; a[-1]",
  ObjectKind::Integer(3)
);

test_eval_ok!(
  array_index_out_of_bounds,
  "const a: i64[3] = [1, 2, 3]; a[3]",
  ObjectKind::Void
);

// --- Tuple Creation & Typing ---

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

// --- Tuple Indexing ---

test_eval_ok!(
  tuple_index_access,
  "const a = [1, \"hello\", 3]; a[0]",
  ObjectKind::Integer(1)
);

test_eval_ok!(
  tuple_index_negative,
  "const a = [1, \"hello\", 3]; a[-1]",
  ObjectKind::Integer(3)
);

test_eval_ok!(
  tuple_index_out_of_bounds,
  "const a = [1, \"hello\", 3]; a[3]",
  ObjectKind::Void
);

// --- Tuple Immutability ---

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
