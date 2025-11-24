use krama_core::object::Object;
use krama_internal::test_eval;

test_eval!(should_eval_bitwise_and_op, "5 & 3", Object::Integer(1));

test_eval!(should_eval_bitwise_or_op, "5 | 3", Object::Integer(7));

test_eval!(should_eval_bitwise_xor_op, "5 ^ 3", Object::Integer(6));

test_eval!(should_eval_bitwise_not_op, "~5", Object::Integer(-6));

test_eval!(should_eval_left_shift_op, "5 << 1", Object::Integer(10));

test_eval!(should_eval_right_shift_op, "5 >> 1", Object::Integer(2));

test_eval!(
  should_eval_bitwise_and_assign_op,
  "let a = 5\na &= 3",
  Object::Integer(1)
);

test_eval!(
  should_eval_bitwise_or_assign_op,
  "let a = 5\na |= 3",
  Object::Integer(7)
);

test_eval!(
  should_eval_bitwise_xor_assign_op,
  "let a = 5\na ^= 3",
  Object::Integer(6)
);

test_eval!(
  should_eval_left_shift_assign_op,
  "let a = 5\na <<= 1",
  Object::Integer(10)
);

test_eval!(
  should_eval_right_shift_assign_op,
  "let a = 5\na >>= 1",
  Object::Integer(2)
);
