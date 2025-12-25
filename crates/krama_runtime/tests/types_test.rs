use krama_core::{ErrorKind, ObjectKind};
use krama_runtime::{test_eval_err, test_eval_ok};

test_eval_ok!(
  type_i8_declaration,
  "const a: i8 = 1\na",
  ObjectKind::Integer(1)
);

test_eval_ok!(
  type_i16_declaration,
  "const a: i16 = 1\na",
  ObjectKind::Integer(1)
);

test_eval_ok!(
  type_i32_declaration,
  "const a: i32 = 1\na",
  ObjectKind::Integer(1)
);

test_eval_ok!(
  type_i64_declaration,
  "const a: i64 = 1\na",
  ObjectKind::Integer(1)
);

test_eval_ok!(
  type_i128_declaration,
  "const a: i128 = 1\na",
  ObjectKind::Integer(1)
);

test_eval_ok!(
  type_isize_declaration,
  "const a: isize = 1\na",
  ObjectKind::Integer(1)
);

test_eval_ok!(
  type_u8_declaration,
  "const a: u8 = 1\na",
  ObjectKind::Integer(1)
);

test_eval_ok!(
  type_u16_declaration,
  "const a: u16 = 1\na",
  ObjectKind::Integer(1)
);

test_eval_ok!(
  type_u32_declaration,
  "const a: u32 = 1\na",
  ObjectKind::Integer(1)
);

test_eval_ok!(
  type_u64_declaration,
  "const a: u64 = 1\na",
  ObjectKind::Integer(1)
);

test_eval_ok!(
  type_u128_declaration,
  "const a: u128 = 1\na",
  ObjectKind::Integer(1)
);

test_eval_ok!(
  type_usize_declaration,
  "const a: usize = 1\na",
  ObjectKind::Integer(1)
);

test_eval_ok!(
  type_f32_declaration,
  "const a: f32 = 1.0\na",
  ObjectKind::Float(1.0)
);

test_eval_ok!(
  type_f64_declaration,
  "const a: f64 = 1.0\na",
  ObjectKind::Float(1.0)
);

test_eval_ok!(
  type_bool_declaration,
  "const a: bool = true\na",
  ObjectKind::Boolean(true)
);

test_eval_ok!(
  type_str_declaration,
  "const a: str = \"hello\"\na",
  ObjectKind::String("hello".to_string())
);

test_eval_ok!(
  type_function_parameter,
  "fn a(b: i8) { b }\na(1)",
  ObjectKind::Integer(1)
);

test_eval_ok!(
  type_function_return,
  "fn a(): i8 { 1 }\na()",
  ObjectKind::Integer(1)
);

test_eval_ok!(
  type_let_statement_inference,
  "let a = 1\na = 2",
  ObjectKind::Integer(2)
);

test_eval_ok!(
  type_let_statement_explicit,
  "let a: i8 = 2\na = 3",
  ObjectKind::Integer(3)
);

test_eval_err!(
  type_error_on_mismatch,
  "const a: i8 = 1.0\na",
  ErrorKind::TypeError(_)
);

test_eval_err!(
  type_arrow_function_parameter_error,
  "const a = (b: i8) => b\na(1.0)",
  ErrorKind::TypeError(_)
);

test_eval_ok!(
  type_custom_alias,
  "type MyInt = i32\nconst a: MyInt = 10\na",
  ObjectKind::Integer(10)
);

test_eval_ok!(
  type_custom_alias_complex,
  "type MyList = i32[]\nconst a: MyList = [1, 2, 3]\na[0]",
  ObjectKind::Integer(1)
);

test_eval_err!(
  type_custom_alias_mismatch,
  "type MyInt = i32\nconst a: MyInt = \"hello\"",
  ErrorKind::TypeError(_)
);

test_eval_ok!(
  type_custom_object,
  r#"
    type User = {
      name: str,
      age: i32
    }

    const a: User = {
      name: "admin",
      age: 25
    }

    a.name == "admin" && a.age == 25
  "#,
  ObjectKind::Boolean(true)
);

test_eval_err!(
  type_custom_object_mismatch,
  r#"
    type User = {
      name: str,
      age: i32
    }

    const a: User = {
      name: "admin",
      age: "25"
    }
  "#,
  ErrorKind::TypeError(_)
);

test_eval_err!(
  type_custom_object_missing_property,
  r#"
    type User = {
      name: str,
      age: i32
    }

    const a: User = {
      name: "admin"
    }
  "#,
  ErrorKind::TypeError(_)
);

test_eval_ok!(
  type_custom_object_optional_property,
  r#"
    type User = {
      name: str,
      age?: i32
    }

    const a: User = {
      name: "admin"
    }

    a.name == "admin"
  "#,
  ObjectKind::Boolean(true)
);
