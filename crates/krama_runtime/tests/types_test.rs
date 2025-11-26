use krama_core::{error::ErrorKind, object::Object};
use krama_internal::{test_eval, test_eval_error};

test_eval!(
  should_i8_type_decl,
  "const a: i8 = 1\n a",
  Object::Integer(1)
);

test_eval!(
  should_i16_type_decl,
  "const a: i16 = 1\n a",
  Object::Integer(1)
);

test_eval!(
  should_i32_type_decl,
  "const a: i32 = 1\n a",
  Object::Integer(1)
);

test_eval!(
  should_i64_type_decl,
  "const a: i64 = 1\n a",
  Object::Integer(1)
);

test_eval!(
  should_i128_type_decl,
  "const a: i128 = 1\n a",
  Object::Integer(1)
);

test_eval!(
  should_isize_type_decl,
  "const a: isize = 1\n a",
  Object::Integer(1)
);

test_eval!(
  should_u8_type_decl,
  "const a: u8 = 1\n a",
  Object::Integer(1)
);

test_eval!(
  should_u16_type_decl,
  "const a: u16 = 1\n a",
  Object::Integer(1)
);

test_eval!(
  should_u32_type_decl,
  "const a: u32 = 1\n a",
  Object::Integer(1)
);

test_eval!(
  should_u64_type_decl,
  "const a: u64 = 1\n a",
  Object::Integer(1)
);

test_eval!(
  should_u128_type_decl,
  "const a: u128 = 1\n a",
  Object::Integer(1)
);

test_eval!(
  should_usize_type_decl,
  "const a: usize = 1\n a",
  Object::Integer(1)
);

test_eval!(
  should_f32_type_decl,
  "const a: f32 = 1.0\n a",
  Object::Float(1.0)
);

test_eval!(
  should_f64_type_decl,
  "const a: f64 = 1.0\n a",
  Object::Float(1.0)
);

test_eval!(
  should_bool_type_decl,
  "const a: bool = true\n a",
  Object::Boolean(true)
);

test_eval!(
  should_str_type_decl,
  "const a: str = \"hello\"\n a",
  Object::String("hello")
);

test_eval!(
  should_fn_param_type,
  "fn a(b: i8) { b }\n a(1)",
  Object::Integer(1)
);

test_eval!(
  should_fn_return_type,
  "fn a(): i8 { 1 }\n a()",
  Object::Integer(1)
);

test_eval!(
  should_let_stmt_type_inference,
  "let a = 1;\na",
  Object::Integer(1)
);

test_eval!(
  should_let_stmt_explicit_type,
  "let a: i8 = 2;\na",
  Object::Integer(2)
);

test_eval_error!(
  should_err_type_mismatch_let_decl,
  "let a: i8 = 1.0;",
  ErrorKind::TypeError(_)
);
