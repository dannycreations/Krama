use krama_core::ObjectKind;
use krama_runtime::test_eval_ok;

test_eval_ok!(
  eval_bang_operator_on_true,
  "!true",
  ObjectKind::Boolean(false)
);

test_eval_ok!(
  eval_bang_operator_on_false,
  "!false",
  ObjectKind::Boolean(true)
);

test_eval_ok!(
  eval_bang_operator_on_number,
  "!5",
  ObjectKind::Boolean(false)
);

test_eval_ok!(
  eval_double_bang_operator_on_true,
  "!!true",
  ObjectKind::Boolean(true)
);

test_eval_ok!(
  eval_double_bang_operator_on_false,
  "!!false",
  ObjectKind::Boolean(false)
);

test_eval_ok!(
  eval_double_bang_operator_on_number,
  "!!5",
  ObjectKind::Boolean(true)
);

test_eval_ok!(
  eval_if_expression_with_true_condition,
  "if (true) { 10 }",
  ObjectKind::Integer(10)
);

test_eval_ok!(
  eval_if_expression_with_false_condition,
  "if (false) { 10 }",
  ObjectKind::Void
);

test_eval_ok!(
  eval_if_expression_with_number_condition,
  "if (1) { 10 }",
  ObjectKind::Integer(10)
);

test_eval_ok!(
  eval_if_expression_with_less_than_condition,
  "if (1 < 2) { 10 }",
  ObjectKind::Integer(10)
);

test_eval_ok!(
  eval_if_expression_with_greater_than_condition,
  "if (1 > 2) { 10 }",
  ObjectKind::Void
);

test_eval_ok!(
  eval_if_else_expression_with_greater_than_condition,
  "if (1 > 2) { 10 } else { 20 }",
  ObjectKind::Integer(20)
);

test_eval_ok!(
  eval_if_else_expression_with_less_than_condition,
  "if (1 < 2) { 10 } else { 20 }",
  ObjectKind::Integer(10)
);

test_eval_ok!(
  eval_if_elif_else_expression,
  "if (1 > 2) { 10 } elif (1 < 2) { 20 } else { 30 }",
  ObjectKind::Integer(20)
);

test_eval_ok!(
  eval_short_circuit_and,
  "false && (1/0)",
  ObjectKind::Boolean(false)
);

test_eval_ok!(
  eval_short_circuit_or,
  "true || (1/0)",
  ObjectKind::Boolean(true)
);
