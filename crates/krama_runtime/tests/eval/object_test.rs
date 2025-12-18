use std::{cell::RefCell, rc::Rc};

use ahash::AHashMap;
use krama_core::Object;
use krama_runtime::test_eval;

test_eval! {
    eval_let_object_expression,
    "let a = { name: \"admin\", age: 20, \"user-id\": 123 }; a",
    Object::Object(Rc::new(RefCell::new({
        let mut map = AHashMap::default();
        map.insert("name", Object::String("admin"));
        map.insert("age", Object::Integer(20));
        map.insert("user-id", Object::Integer(123));
        map
    })))
}

test_eval! {
    eval_const_object_expression_with_literal_key_and_trailing,
    "const a = { name: \"admin\", age: 20, \"user-id\": 123, }; a",
    Object::Object(Rc::new(RefCell::new({
        let mut map = AHashMap::default();
        map.insert("name", Object::String("admin"));
        map.insert("age", Object::Integer(20));
        map.insert("user-id", Object::Integer(123));
        map
    })))
}

test_eval! {
    eval_object_property_access,
    "let a = { name: \"admin\", age: 20 }; a.name",
    Object::String("admin")
}

test_eval! {
    eval_object_index_access,
    "let a = { name: \"admin\", age: 20 }; a[\"age\"]",
    Object::Integer(20)
}

test_eval! {
    eval_object_property_assignment,
    "let a = { name: \"admin\", age: 20 }; a.name = \"guest\"; a.name",
    Object::String("guest")
}

test_eval! {
    eval_object_index_assignment,
    "let a = { name: \"admin\", \"user-id\": 20 }; a[\"user-id\"] = 30; a[\"user-id\"]",
    Object::Integer(30)
}

test_eval! {
    eval_object_property_add_assign,
    "let a = { score: 10 }; a.score += 5; a.score",
    Object::Integer(15)
}

test_eval! {
    eval_object_property_increment,
    "let a = { score: 10 }; a.score++; a.score",
    Object::Integer(11)
}

test_eval! {
    eval_nested_object_access,
    "let a = { user: { name: \"admin\" } }; a.user.name",
    Object::String("admin")
}

test_eval! {
    eval_nested_object_assignment,
    "let a = { user: { name: \"admin\" } }; a.user.name = \"guest\"; a.user.name",
    Object::String("guest")
}
