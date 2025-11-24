use futures::future::FutureExt;
use krama_core::object::Object;
use krama_internal::test_eval_async;

test_eval_async!(
  should_call_native_fn,
  r#"print("hello world")"#,
  Object::Void
);

test_eval_async!(
  should_call_user_defined_fn,
  r#"
        fn identity(x) { x }
        identity(5)
    "#,
  Object::Integer(5)
);

test_eval_async!(
  should_call_let_bound_fn,
  r#"
        let identity = fn(x) { x }
        identity(5)
    "#,
  Object::Integer(5)
);

test_eval_async!(
  should_call_const_bound_fn,
  r#"
        const identity = fn(x) { x }
        identity(5)
    "#,
  Object::Integer(5)
);

test_eval_async!(
  should_call_arrow_fn,
  r#"
        let identity = fn(x) => x
        identity(5)
    "#,
  Object::Integer(5)
);

test_eval_async!(
  should_handle_return_stmt_in_fn,
  r#"
        fn add(x, y) { return x + y }
        add(5, 5)
    "#,
  Object::Integer(10)
);
