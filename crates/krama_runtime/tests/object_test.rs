use krama_core::{ErrorKind, ObjectKind};
use krama_runtime::{test_eval_err, test_eval_match, test_eval_ok};

test_eval_match! {
  object_let_expression,
  "let a = { name: \"admin\", age: 20, \"user-id\": 123 }; a = {}",
  ObjectKind::Object { .. }
}

test_eval_match! {
  object_const_expression_with_literal_key_and_trailing,
  "const a = { name: \"admin\", age: 20, \"user-id\": 123, }; a",
  ObjectKind::Object { .. }
}

test_eval_ok! {
  object_property_access,
  "const a = { name: \"admin\", age: 20 }; a.name",
  ObjectKind::String("admin")
}

test_eval_ok! {
  object_index_access,
  "const a = { name: \"admin\", age: 20 }; a[\"age\"]",
  ObjectKind::Integer(20)
}

test_eval_ok! {
  object_property_assignment,
  "let a = { name: \"admin\", age: 20 }; a.name = \"guest\"; a.name",
  ObjectKind::String("guest")
}

test_eval_ok! {
  object_index_assignment,
  "let a = { name: \"admin\", \"user-id\": 20 }; a[\"user-id\"] = 30; a[\"user-id\"]",
  ObjectKind::Integer(30)
}

test_eval_ok! {
  object_property_add_assignment,
  "let a = { score: 10 }; a.score += 5; a.score",
  ObjectKind::Integer(15)
}

test_eval_ok! {
  object_property_increment,
  "let a = { score: 10 }; a.score++; a.score",
  ObjectKind::Integer(11)
}

test_eval_ok! {
  object_nested_access,
  "const a = { user: { name: \"admin\" } }; a.user.name",
  ObjectKind::String("admin")
}

test_eval_ok! {
  object_nested_assignment,
  "let a = { user: { name: \"admin\" } }; a.user.name = \"guest\"; a.user.name",
  ObjectKind::String("guest")
}

test_eval_err! {
  object_const_immutability,
  "const a = { name: \"admin\" }; a.name = \"guest\"; a.name",
  ErrorKind::TypeError(_)
}

test_eval_err! {
  object_const_index_immutability,
  "const a = { name: \"admin\" }; a[\"name\"] = \"guest\"; a[\"name\"]",
  ErrorKind::TypeError(_)
}

test_eval_err! {
  object_const_property_increment_immutability,
  "const a = { score: 10 }; a.score++; a.score",
  ErrorKind::TypeError(_)
}

test_eval_ok!(
  object_key_in,
  r#"
    const o = { a: 1, b: 2 }
    "a" in o
  "#,
  ObjectKind::Boolean(true)
);

test_eval_ok!(
  object_key_not_in,
  r#"
    const o = { a: 1, b: 2 }
    "c" in o
  "#,
  ObjectKind::Boolean(false)
);
