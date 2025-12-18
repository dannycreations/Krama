use krama_core::Object;
use krama_runtime::test_eval;

test_eval!(
  eval_array_index,
  r#"const a: i64[3] = [1, 2, 3]; a[0]"#,
  Object::Integer(1)
);

test_eval!(
  eval_array_index_negative,
  r#"const a: i64[3] = [1, 2, 3]; a[-1]"#,
  Object::Integer(3)
);

test_eval!(
  eval_array_index_out_of_bounds,
  r#"const a: i64[3] = [1, 2, 3]; a[3]"#,
  Object::Void
);

test_eval!(
  eval_tuple_index,
  r#"const a = [1, "hello", 3]; a[0]"#,
  Object::Integer(1)
);

test_eval!(
  eval_tuple_index_negative,
  r#"const a = [1, "hello", 3]; a[-1]"#,
  Object::Integer(3)
);

test_eval!(
  eval_tuple_index_out_of_bounds,
  r#"const a = [1, "hello", 3]; a[3]"#,
  Object::Void
);

test_eval!(
  eval_string_index,
  r#"const a = "hello"; a[0]"#,
  Object::String("h")
);

test_eval!(
  eval_string_index_negative,
  r#"const a = "hello"; a[-1]"#,
  Object::String("o")
);

test_eval!(
  eval_string_index_out_of_bounds,
  r#"const a = "hello"; a[5]"#,
  Object::Void
);
