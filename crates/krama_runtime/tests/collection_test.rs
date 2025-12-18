use bumpalo::Bump;
use krama_core::{ErrorKind, Object};
use krama_runtime::{test_eval_err, test_eval_ok, Interpreter};

macro_rules! test_eval_to_string {
  ($name:ident, $source:expr, $expected:expr) => {
    #[tokio::test]
    async fn $name() {
      let arena = Bump::new();
      let interpreter = Interpreter::new(&arena, None);
      let source_in_arena = arena.alloc_str($source);
      let result = interpreter.eval(source_in_arena).await.unwrap();
      assert_eq!(result.to_string(), $expected);
    }
  };
}

test_eval_to_string!(eval_empty_array, "[]", "[]");

test_eval_to_string!(eval_tuple, "[1, true, \"hello\"]", "[1, true, hello]");

test_eval_to_string!(
  eval_nested_tuple,
  "[1, [true, \"hello\"]]",
  "[1, [true, hello]]"
);

test_eval_ok!(eval_typed_array, "const a: i32[] = [1, 2, 3]", Object::Void);

test_eval_ok!(
  eval_tuple_with_type_annotation,
  "const a: [i32, bool] = [1, true]",
  Object::Void
);

test_eval_ok!(
  eval_fixed_length_array,
  "const a: i32[3] = [1, 2, 3]",
  Object::Void
);

test_eval_ok!(
  eval_fixed_length_array_with_less_elements,
  "const a: i32[3] = [1, 2]",
  Object::Void
);

test_eval_err!(
  eval_fixed_length_array_with_more_elements,
  "const a: i32[3] = [1, 2, 3, 4]",
  ErrorKind::TypeError(_)
);

test_eval_err!(
  eval_typed_array_with_mixed_types,
  "const a: i32[] = [1, true, 3]",
  ErrorKind::TypeError(_)
);

test_eval_err!(
  eval_tuple_with_wrong_type,
  "const a: [i32, bool] = [1, 1]",
  ErrorKind::TypeError(_)
);

test_eval_err!(
  eval_tuple_with_wrong_length,
  "const a: [i32, bool] = [1]",
  ErrorKind::TypeError(_)
);
