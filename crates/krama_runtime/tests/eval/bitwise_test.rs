use krama_core::Object;
use krama_runtime::test_eval;

test_eval!(eval_bitwise_and, "5 & 3", Object::Integer(1));

test_eval!(eval_bitwise_or, "5 | 3", Object::Integer(7));

test_eval!(eval_bitwise_xor, "5 ^ 3", Object::Integer(6));

test_eval!(eval_bitwise_not, "~5", Object::Integer(-6));

test_eval!(eval_left_shift, "5 << 1", Object::Integer(10));

test_eval!(eval_right_shift, "5 >> 1", Object::Integer(2));

test_eval!(
  eval_bitwise_and_assignment,
  "let a = 5\na &= 3",
  Object::Integer(1)
);

test_eval!(
  eval_bitwise_or_assignment,
  "let a = 5\na |= 3",
  Object::Integer(7)
);

test_eval!(
  eval_bitwise_xor_assignment,
  "let a = 5\na ^= 3",
  Object::Integer(6)
);

test_eval!(
  eval_left_shift_assignment,
  "let a = 5\na <<= 1",
  Object::Integer(10)
);

test_eval!(
  eval_right_shift_assignment,
  "let a = 5\na >>= 1",
  Object::Integer(2)
);
