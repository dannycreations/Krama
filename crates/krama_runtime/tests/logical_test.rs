use krama_core::object::Object;
use krama_internal::test_eval;

test_eval!(eval_bang_operator_on_true, "!true", Object::Boolean(false));

test_eval!(eval_bang_operator_on_false, "!false", Object::Boolean(true));

test_eval!(eval_bang_operator_on_number, "!5", Object::Boolean(false));

test_eval!(
  eval_double_bang_operator_on_true,
  "!!true",
  Object::Boolean(true)
);

test_eval!(
  eval_double_bang_operator_on_false,
  "!!false",
  Object::Boolean(false)
);

test_eval!(
  eval_double_bang_operator_on_number,
  "!!5",
  Object::Boolean(true)
);

test_eval!(
  eval_if_expression_with_true_condition,
  "if (true) { 10 }",
  Object::Integer(10)
);

test_eval!(
  eval_if_expression_with_false_condition,
  "if (false) { 10 }",
  Object::Void
);

test_eval!(
  eval_if_expression_with_number_condition,
  "if (1) { 10 }",
  Object::Integer(10)
);

test_eval!(
  eval_if_expression_with_less_than_condition,
  "if (1 < 2) { 10 }",
  Object::Integer(10)
);

test_eval!(
  eval_if_expression_with_greater_than_condition,
  "if (1 > 2) { 10 }",
  Object::Void
);

test_eval!(
  eval_if_else_expression_with_greater_than_condition,
  "if (1 > 2) { 10 } else { 20 }",
  Object::Integer(20)
);

test_eval!(
  eval_if_else_expression_with_less_than_condition,
  "if (1 < 2) { 10 } else { 20 }",
  Object::Integer(10)
);

test_eval!(
  eval_if_elif_else_expression,
  "if (1 > 2) { 10 } elif (1 < 2) { 20 } else { 30 }",
  Object::Integer(20)
);

test_eval!(
  eval_short_circuit_and,
  "false && (1/0)",
  Object::Boolean(false)
);

test_eval!(
  eval_short_circuit_or,
  "true || (1/0)",
  Object::Boolean(true)
);
