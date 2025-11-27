use bumpalo::Bump;
use krama_runtime::interpreter::Interpreter;

macro_rules! test_index {
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

test_index!(
  should_array_index_0,
  r#"const a: i64[3] = [1, 2, 3]; a[0]"#,
  "1"
);
test_index!(
  should_array_index_1,
  r#"const a: i64[3] = [1, 2, 3]; a[1]"#,
  "2"
);
test_index!(
  should_array_index_2,
  r#"const a: i64[3] = [1, 2, 3]; a[2]"#,
  "3"
);
test_index!(
  should_array_index_negative_1,
  r#"const a: i64[3] = [1, 2, 3]; a[-1]"#,
  "3"
);
test_index!(
  should_array_index_negative_2,
  r#"const a: i64[3] = [1, 2, 3]; a[-2]"#,
  "2"
);
test_index!(
  should_array_index_out_of_bounds,
  r#"const a: i64[3] = [1, 2, 3]; a[3]"#,
  "void"
);

test_index!(
  should_tuple_index_0,
  r#"const a = [1, "hello", 3]; a[0]"#,
  "1"
);
test_index!(
  should_tuple_index_1,
  r#"const a = [1, "hello", 3]; a[1]"#,
  "hello"
);
test_index!(
  should_tuple_index_2,
  r#"const a = [1, "hello", 3]; a[2]"#,
  "3"
);
test_index!(
  should_tuple_index_negative_1,
  r#"const a = [1, "hello", 3]; a[-1]"#,
  "3"
);
test_index!(
  should_tuple_index_negative_2,
  r#"const a = [1, "hello", 3]; a[-2]"#,
  "hello"
);
test_index!(
  should_tuple_index_out_of_bounds,
  r#"const a = [1, "hello", 3]; a[3]"#,
  "void"
);

test_index!(should_string_index_0, r#"const a = "hello"; a[0]"#, "h");
test_index!(should_string_index_1, r#"const a = "hello"; a[1]"#, "e");
test_index!(should_string_index_4, r#"const a = "hello"; a[4]"#, "o");
test_index!(
  should_string_index_negative_1,
  r#"const a = "hello"; a[-1]"#,
  "o"
);
test_index!(
  should_string_index_negative_2,
  r#"const a = "hello"; a[-2]"#,
  "l"
);
test_index!(
  should_string_index_out_of_bounds,
  r#"const a = "hello"; a[5]"#,
  "void"
);
