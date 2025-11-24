use bumpalo::Bump;
use krama_runtime::interpreter::Interpreter;

macro_rules! test_collection {
  ($name:ident, $source:expr, $expected:expr) => {
    #[tokio::test]
    async fn $name() {
      let arena = Bump::new();
      let interpreter = Interpreter::new(&arena, None);
      let result = interpreter.eval($source).await.unwrap();
      assert_eq!(result.to_string(), $expected);
    }
  };
}

macro_rules! test_collection_ok {
  ($name:ident, $source:expr) => {
    #[tokio::test]
    async fn $name() {
      let arena = Bump::new();
      let interpreter = Interpreter::new(&arena, None);
      let result = interpreter.eval($source).await;
      assert!(result.is_ok());
    }
  };
}

macro_rules! test_collection_err {
  ($name:ident, $source:expr) => {
    #[tokio::test]
    async fn $name() {
      let arena = Bump::new();
      let interpreter = Interpreter::new(&arena, None);
      let result = interpreter.eval($source).await;
      assert!(result.is_err());
    }
  };
}

test_collection!(should_eval_empty_array, "[]", "[]");

test_collection!(
  should_eval_tuple,
  "[1, true, \"hello\"]",
  "[1, true, hello]"
);

test_collection!(
  should_eval_nested_tuple,
  "[1, [true, \"hello\"]]",
  "[1, [true, hello]]"
);

test_collection_ok!(should_eval_typed_array, "const a: i32[] = [1, 2, 3]");

test_collection_ok!(
  should_eval_tuple_with_type_annotation,
  "const a: [i32, bool] = [1, true]"
);

test_collection_ok!(
  should_eval_fixed_length_array,
  "const a: i32[3] = [1, 2, 3]"
);

test_collection_ok!(
  should_eval_fixed_length_array_with_less_elements,
  "const a: i32[3] = [1, 2]"
);

test_collection_err!(
  should_panic_on_fixed_length_array_with_more_elements,
  "const a: i32[3] = [1, 2, 3, 4]"
);

test_collection_err!(
  should_panic_on_typed_array_with_mixed_types,
  "const a: i32[] = [1, true, 3]"
);

test_collection_err!(
  should_panic_on_tuple_with_wrong_type,
  "const a: [i32, bool] = [1, 1]"
);

test_collection_err!(
  should_panic_on_tuple_with_wrong_length,
  "const a: [i32, bool] = [1]"
);
