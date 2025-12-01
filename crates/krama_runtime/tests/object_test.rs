use krama_core::object::Object;
use krama_runtime::test_eval;
use rustc_hash::FxHashMap;

test_eval! {
    eval_let_object_expression,
    "let a = { name: \"admin\", age: 20 }; a",
    Object::Object({
        let mut map = FxHashMap::default();
        map.insert("name", Object::String("admin"));
        map.insert("age", Object::Integer(20));
        map
    })
}

test_eval! {
    eval_const_object_expression,
    "const a = { name: \"admin\", age: 20 }; a",
    Object::Object({
        let mut map = FxHashMap::default();
        map.insert("name", Object::String("admin"));
        map.insert("age", Object::Integer(20));
        map
    })
}
