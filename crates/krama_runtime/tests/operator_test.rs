use krama_core::{ErrorKind, Object};
use krama_runtime::{test_eval_err, test_eval_ok};

// --- Arithmetic Operators ---

test_eval_ok!(operator_integer_addition, "5 + 5", Object::Integer(10));

test_eval_ok!(operator_integer_subtraction, "5 - 5", Object::Integer(0));

test_eval_ok!(
  operator_integer_multiplication,
  "5 * 5",
  Object::Integer(25)
);

test_eval_ok!(operator_integer_division, "5 / 5", Object::Integer(1));

test_eval_ok!(operator_integer_modulo, "5 % 2", Object::Integer(1));

test_eval_ok!(
  operator_integer_exponentiation,
  "2 ** 3",
  Object::Integer(8)
);

test_eval_ok!(operator_float_addition, "5.0 + 5.0", Object::Float(10.0));

test_eval_ok!(operator_float_subtraction, "5.0 - 5.0", Object::Float(0.0));

test_eval_ok!(
  operator_float_multiplication,
  "5.0 * 5.0",
  Object::Float(25.0)
);

test_eval_ok!(operator_float_division, "5.0 / 5.0", Object::Float(1.0));

test_eval_ok!(operator_float_modulo, "5.0 % 2.0", Object::Float(1.0));

test_eval_ok!(
  operator_float_exponentiation,
  "2.0 ** 3.0",
  Object::Float(8.0)
);

// --- Precedence & Complex Expressions ---

test_eval_ok!(
  operator_precedence_multiplication_addition,
  "2 + 3 * 4",
  Object::Integer(14)
);

test_eval_ok!(
  operator_precedence_parentheses,
  "(2 + 3) * 4",
  Object::Integer(20)
);

test_eval_ok!(
  operator_precedence_exponentiation,
  "2 * 3 ** 2",
  Object::Integer(18)
);

test_eval_ok!(
  operator_precedence_comparison_logical,
  "1 + 2 == 3 && 4 > 5 == false",
  Object::Bool(true)
);

// --- Edge Cases ---

test_eval_err!(
  operator_division_by_zero_integer,
  "1 / 0",
  ErrorKind::RuntimeError(_)
);

test_eval_ok!(
  operator_division_by_zero_float,
  "1.0 / 0.0",
  Object::Float(f64::INFINITY)
);

// --- Unary Operators ---

test_eval_ok!(
  operator_prefix_increment,
  "let a = 5; ++a",
  Object::Integer(6)
);

test_eval_ok!(
  operator_postfix_increment,
  "let a = 5; a++",
  Object::Integer(5)
);

test_eval_ok!(
  operator_prefix_decrement,
  "let a = 5; --a",
  Object::Integer(4)
);

test_eval_ok!(
  operator_postfix_decrement,
  "let a = 5; a--",
  Object::Integer(5)
);

test_eval_ok!(operator_logical_bang_true, "!true", Object::Bool(false));

test_eval_ok!(operator_logical_bang_false, "!false", Object::Bool(true));

test_eval_ok!(operator_logical_bang_number, "!5", Object::Bool(false));

test_eval_ok!(
  operator_logical_double_bang_true,
  "!!true",
  Object::Bool(true)
);

test_eval_ok!(
  operator_logical_double_bang_number,
  "!!5",
  Object::Bool(true)
);

// --- Assignment Operators ---

test_eval_ok!(operator_assignment, "let a = 5; a = 2", Object::Integer(2));

test_eval_ok!(
  operator_plus_equal_assignment,
  "let a = 5; a += 5",
  Object::Integer(10)
);

test_eval_ok!(
  operator_minus_equal_assignment,
  "let a = 5; a -= 5",
  Object::Integer(0)
);

test_eval_ok!(
  operator_star_equal_assignment,
  "let a = 5; a *= 5",
  Object::Integer(25)
);

test_eval_ok!(
  operator_slash_equal_assignment,
  "let a = 5; a /= 5",
  Object::Integer(1)
);

test_eval_ok!(
  operator_percent_equal_assignment,
  "let a = 5; a %= 2",
  Object::Integer(1)
);

test_eval_err!(
  operator_assignment_to_unknown,
  "y = 10",
  ErrorKind::ReferenceError(_)
);

// --- Comparison Operators ---

test_eval_ok!(
  operator_string_equality,
  "\"a\" == \"a\"",
  Object::Bool(true)
);

test_eval_ok!(
  operator_string_inequality,
  "\"a\" != \"b\"",
  Object::Bool(true)
);

test_eval_ok!(
  operator_boolean_equality,
  "true == true",
  Object::Bool(true)
);

test_eval_ok!(
  operator_boolean_inequality,
  "true != false",
  Object::Bool(true)
);

test_eval_err!(
  operator_invalid_string_multiplication,
  "\"a\" * \"b\"",
  ErrorKind::TypeError(_)
);

test_eval_err!(
  operator_invalid_boolean_addition,
  "true + false",
  ErrorKind::TypeError(_)
);
