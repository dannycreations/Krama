use krama_core::{ErrorKind, Object};
use krama_runtime::{test_eval_err, test_eval_ok};

// --- Basic Function Invoke ---

test_eval_ok!(function_invoke, "print(\"hello world\")", Object::Void);

test_eval_ok!(
  function_with_statement,
  "fn a(x, y) { x + y }; a(5, 5)",
  Object::Integer(10)
);

test_eval_ok!(
  function_with_return_statement,
  "fn a(x, y) { return x + y }; a(5, 5)",
  Object::Integer(10)
);

// --- Anonymous Functions & Arrows ---

test_eval_ok!(
  function_let_bound,
  "let a = fn(x) { x }; a = a(5)",
  Object::Integer(5)
);

test_eval_ok!(
  function_const_bound,
  "const a = fn(x) { x }; a(5)",
  Object::Integer(5)
);

test_eval_ok!(
  function_arrow,
  "const a = (x) => x; a(5)",
  Object::Integer(5)
);

test_eval_err!(
  function_arrow_block,
  "const a = (x) => { x }; a(5)",
  ErrorKind::SyntaxError(_)
);

test_eval_err!(
  function_arrow_classic,
  "const a = fn(x) => x; a(5)",
  ErrorKind::SyntaxError(_)
);

// --- Arguments & Defaults ---

test_eval_ok!(
  function_with_default_argument,
  "fn a(x = 0) { x }; a()",
  Object::Integer(0)
);

test_eval_ok!(
  function_with_passed_argument,
  "fn a(x = 10) { x }; a(1)",
  Object::Integer(1)
);

test_eval_ok!(
  function_arrow_with_default_argument,
  "const a = (x = 0) => x; a()",
  Object::Integer(0)
);

test_eval_ok!(
  function_arrow_with_passed_argument,
  "const a = (x = 0) => x; a(1)",
  Object::Integer(1)
);

test_eval_ok!(
  function_arrow_with_optional_argument,
  "const a = (x, y = 0) => x + 0; a(1)",
  Object::Integer(1)
);

test_eval_err!(
  function_missing_required_argument,
  "fn a(x, y) { x }; a(1)",
  ErrorKind::TypeError(_)
);

test_eval_err!(
  function_missing_argument_after_optional,
  "fn a(x = 0, y) { x }; a()",
  ErrorKind::TypeError(_)
);

// --- Closures & Scoping ---

test_eval_ok!(
  function_closure_capture,
  r#"
    let x = 10
    const add_x = (y) => x + y
    add_x(5)
  "#,
  Object::Integer(15)
);

test_eval_ok!(
  function_closure_mutation,
  r#"
    let x = 10
    const inc = fn() { x = x + 1; x }
    inc()
    x
  "#,
  Object::Integer(11)
);

// --- Recursion ---

test_eval_ok!(
  function_recursion,
  r#"
    fn fib(n) {
      if (n <= 1) { n } else { fib(n - 1) + fib(n - 2) }
    }
    fib(7)
  "#,
  Object::Integer(13)
);
