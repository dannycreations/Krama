use krama_core::object::Object;
use krama_internal::test_eval;

test_eval!(should_eval_bang_op_true, "!true", Object::Boolean(false));

test_eval!(should_eval_bang_op_false, "!false", Object::Boolean(true));

test_eval!(should_eval_bang_op_num, "!5", Object::Boolean(false));

test_eval!(
  should_eval_double_bang_op_true,
  "!!true",
  Object::Boolean(true)
);

test_eval!(
  should_eval_double_bang_op_false,
  "!!false",
  Object::Boolean(false)
);

test_eval!(should_eval_double_bang_op_num, "!!5", Object::Boolean(true));

test_eval!(
  should_eval_if_expr_true_cond,
  "if (true) { 10 }",
  Object::Integer(10)
);

test_eval!(
  should_eval_if_expr_false_cond,
  "if (false) { 10 }",
  Object::Void
);

test_eval!(
  should_eval_if_expr_num_cond,
  "if (1) { 10 }",
  Object::Integer(10)
);

test_eval!(
  should_eval_if_expr_less_than_cond,
  "if (1 < 2) { 10 }",
  Object::Integer(10)
);

test_eval!(
  should_eval_if_expr_greater_than_cond,
  "if (1 > 2) { 10 }",
  Object::Void
);

test_eval!(
  should_eval_if_else_expr_greater_than_cond,
  "if (1 > 2) { 10 } else { 20 }",
  Object::Integer(20)
);

test_eval!(
  should_eval_if_else_expr_less_than_cond,
  "if (1 < 2) { 10 } else { 20 }",
  Object::Integer(10)
);

test_eval!(
  should_eval_if_elif_else_expr,
  "if (1 > 2) { 10 } elif (1 < 2) { 20 } else { 30 }",
  Object::Integer(20)
);

test_eval!(
  should_short_circuit_and,
  "false && (1/0)",
  Object::Boolean(false)
);

test_eval!(
  should_short_circuit_or,
  "true || (1/0)",
  Object::Boolean(true)
);
