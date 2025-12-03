use ahash::AHashMap;
use krama_core::object::Object;
use krama_runtime::test_eval;

test_eval! {
    eval_let_object_expression,
    "let a = { name: \"admin\", age: 20, \"user-id\": 123 }; a",
    Object::Object({
        let mut map = AHashMap::default();
        map.insert("name", Object::String("admin"));
        map.insert("age", Object::Integer(20));
        map.insert("user-id", Object::Integer(123));
        map
    })
}

test_eval! {
    eval_const_object_expression_with_literal_key_and_trailing,
    "const a = { name: \"admin\", age: 20, \"user-id\": 123, }; a",
    Object::Object({
        let mut map = AHashMap::default();
        map.insert("name", Object::String("admin"));
        map.insert("age", Object::Integer(20));
        map.insert("user-id", Object::Integer(123));
        map
    })
}
