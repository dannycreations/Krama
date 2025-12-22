use krama_core::ObjectKind;
use krama_runtime::test_eval_ok;

test_eval_ok!(eval_integer_addition, "5 + 5", ObjectKind::Integer(10));

test_eval_ok!(eval_integer_subtraction, "5 - 5", ObjectKind::Integer(0));

test_eval_ok!(
  eval_integer_multiplication,
  "5 * 5",
  ObjectKind::Integer(25)
);

test_eval_ok!(eval_integer_division, "5 / 5", ObjectKind::Integer(1));

test_eval_ok!(eval_integer_modulo, "5 % 2", ObjectKind::Integer(1));

test_eval_ok!(
  eval_integer_exponentiation,
  "2 ** 3",
  ObjectKind::Integer(8)
);

test_eval_ok!(eval_float_addition, "5.0 + 5.0", ObjectKind::Float(10.0));

test_eval_ok!(eval_float_subtraction, "5.0 - 5.0", ObjectKind::Float(0.0));

test_eval_ok!(
  eval_float_multiplication,
  "5.0 * 5.0",
  ObjectKind::Float(25.0)
);

test_eval_ok!(eval_float_division, "5.0 / 5.0", ObjectKind::Float(1.0));

test_eval_ok!(eval_float_modulo, "5.0 % 2.0", ObjectKind::Float(1.0));

test_eval_ok!(
  eval_float_exponentiation,
  "2.0 ** 3.0",
  ObjectKind::Float(8.0)
);

test_eval_ok!(
  eval_prefix_increment,
  "let a = 5; ++a",
  ObjectKind::Integer(6)
);

test_eval_ok!(
  eval_postfix_increment,
  "let a = 5; a++",
  ObjectKind::Integer(5)
);

test_eval_ok!(
  eval_prefix_decrement,
  "let a = 5; --a",
  ObjectKind::Integer(4)
);

test_eval_ok!(
  eval_postfix_decrement,
  "let a = 5; a--",
  ObjectKind::Integer(5)
);

test_eval_ok!(
  eval_plus_equal_assignment,
  "let a = 5; a += 5",
  ObjectKind::Integer(10)
);

test_eval_ok!(
  eval_minus_equal_assignment,
  "let a = 5; a -= 5",
  ObjectKind::Integer(0)
);

test_eval_ok!(
  eval_star_equal_assignment,
  "let a = 5; a *= 5",
  ObjectKind::Integer(25)
);

test_eval_ok!(
  eval_slash_equal_assignment,
  "let a = 5; a /= 5",
  ObjectKind::Integer(1)
);

test_eval_ok!(
  eval_percent_equal_assignment,
  "let a = 5; a %= 2",
  ObjectKind::Integer(1)
);
