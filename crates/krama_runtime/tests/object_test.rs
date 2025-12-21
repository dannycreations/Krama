use krama_core::{ErrorKind, Object};
use krama_runtime::{test_eval_err, test_eval_match, test_eval_ok};

test_eval_match! {
  eval_let_object_expression,
  "let a = { name: \"admin\", age: 20, \"user-id\": 123 }; a = {}",
  Object::Object { .. }
}

test_eval_match! {
  eval_const_object_expression_with_literal_key_and_trailing,
  "const a = { name: \"admin\", age: 20, \"user-id\": 123, }; a",
  Object::Object { .. }
}

test_eval_ok! {
  eval_object_property_access,
  "const a = { name: \"admin\", age: 20 }; a.name",
  Object::String("admin")
}

test_eval_ok! {
  eval_object_index_access,
  "const a = { name: \"admin\", age: 20 }; a[\"age\"]",
  Object::Integer(20)
}

test_eval_ok! {
  eval_object_property_assignment,
  "let a = { name: \"admin\", age: 20 }; a.name = \"guest\"; a.name",
  Object::String("guest")
}

test_eval_ok! {
  eval_object_index_assignment,
  "let a = { name: \"admin\", \"user-id\": 20 }; a[\"user-id\"] = 30; a[\"user-id\"]",
  Object::Integer(30)
}

test_eval_ok! {
  eval_object_property_add_assign,
  "let a = { score: 10 }; a.score += 5; a.score",
  Object::Integer(15)
}

test_eval_ok! {
  eval_object_property_increment,
  "let a = { score: 10 }; a.score++; a.score",
  Object::Integer(11)
}

test_eval_ok! {
  eval_nested_object_access,
  "const a = { user: { name: \"admin\" } }; a.user.name",
  Object::String("admin")
}

test_eval_ok! {
  eval_nested_object_assignment,
  "let a = { user: { name: \"admin\" } }; a.user.name = \"guest\"; a.user.name",
  Object::String("guest")
}

test_eval_err! {
  eval_const_object_immutability,
  "const a = { name: \"admin\" }; a.name = \"guest\"; a.name",
  ErrorKind::TypeError(_)
}

test_eval_err! {
  eval_const_object_index_immutability,
  "const a = { name: \"admin\" }; a[\"name\"] = \"guest\"; a[\"name\"]",
  ErrorKind::TypeError(_)
}

test_eval_err! {
  eval_const_object_property_increment_immutability,
  "const a = { score: 10 }; a.score++; a.score",
  ErrorKind::TypeError(_)
}
