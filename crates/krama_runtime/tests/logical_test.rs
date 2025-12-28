use krama_core::Object;
use krama_runtime::test_eval_ok;

// --- Logical Control Flow & Short Circuiting ---

test_eval_ok!(
  logical_short_circuit_and,
  "false && (1/0)",
  Object::Bool(false)
);

test_eval_ok!(
  logical_short_circuit_or,
  "true || (1/0)",
  Object::Bool(true)
);

test_eval_ok!(
  logical_if_expression_true,
  "if (true) { 10 }",
  Object::Integer(10)
);

test_eval_ok!(
  logical_if_expression_false,
  "if (false) { 10 }",
  Object::Void
);

test_eval_ok!(
  logical_if_expression_truthy_number,
  "if (1) { 10 }",
  Object::Integer(10)
);

test_eval_ok!(
  logical_if_else_expression,
  "if (1 > 2) { 10 } else { 20 }",
  Object::Integer(20)
);

test_eval_ok!(
  logical_if_elif_else_expression,
  "if (1 > 2) { 10 } elif (1 < 2) { 20 } else { 30 }",
  Object::Integer(20)
);
