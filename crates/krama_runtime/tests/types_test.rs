use krama_core::{ErrorKind, Object};
use krama_runtime::{test_eval_err, test_eval_ok};

test_eval_ok!(
  eval_i8_type_declaration,
  "const a: i8 = 1\na",
  Object::Integer(1)
);

test_eval_ok!(
  eval_i16_type_declaration,
  "const a: i16 = 1\na",
  Object::Integer(1)
);

test_eval_ok!(
  eval_i32_type_declaration,
  "const a: i32 = 1\na",
  Object::Integer(1)
);

test_eval_ok!(
  eval_i64_type_declaration,
  "const a: i64 = 1\na",
  Object::Integer(1)
);

test_eval_ok!(
  eval_i128_type_declaration,
  "const a: i128 = 1\na",
  Object::Integer(1)
);

test_eval_ok!(
  eval_isize_type_declaration,
  "const a: isize = 1\na",
  Object::Integer(1)
);

test_eval_ok!(
  eval_u8_type_declaration,
  "const a: u8 = 1\na",
  Object::Integer(1)
);

test_eval_ok!(
  eval_u16_type_declaration,
  "const a: u16 = 1\na",
  Object::Integer(1)
);

test_eval_ok!(
  eval_u32_type_declaration,
  "const a: u32 = 1\na",
  Object::Integer(1)
);

test_eval_ok!(
  eval_u64_type_declaration,
  "const a: u64 = 1\na",
  Object::Integer(1)
);

test_eval_ok!(
  eval_u128_type_declaration,
  "const a: u128 = 1\na",
  Object::Integer(1)
);

test_eval_ok!(
  eval_usize_type_declaration,
  "const a: usize = 1\na",
  Object::Integer(1)
);

test_eval_ok!(
  eval_f32_type_declaration,
  "const a: f32 = 1.0\na",
  Object::Float(1.0)
);

test_eval_ok!(
  eval_f64_type_declaration,
  "const a: f64 = 1.0\na",
  Object::Float(1.0)
);

test_eval_ok!(
  eval_bool_type_declaration,
  "const a: bool = true\na",
  Object::Boolean(true)
);

test_eval_ok!(
  eval_str_type_declaration,
  "const a: str = \"hello\"\na",
  Object::String("hello")
);

test_eval_ok!(
  eval_function_parameter_type,
  "fn a(b: i8) { b }\na(1)",
  Object::Integer(1)
);

test_eval_ok!(
  eval_function_return_type,
  "fn a(): i8 { 1 }\na()",
  Object::Integer(1)
);

test_eval_ok!(
  eval_let_statement_type_inference,
  "let a = 1\na = 2",
  Object::Integer(2)
);

test_eval_ok!(
  eval_let_statement_explicit_type,
  "let a: i8 = 2\na = 3",
  Object::Integer(3)
);

test_eval_err!(
  eval_error_on_type_mismatch,
  "const a: i8 = 1.0\na",
  ErrorKind::TypeError(_)
);

test_eval_err!(
  eval_arrow_function_parameter_type_error,
  "const a = (b: i8) => b\na(1.0)",
  ErrorKind::TypeError(_)
);
