use krama_core::Object;
use krama_runtime::test_eval_ok;

test_eval_ok!(bitwise_and, "5 & 3", Object::Integer(1));

test_eval_ok!(bitwise_or, "5 | 3", Object::Integer(7));

test_eval_ok!(bitwise_xor, "5 ^ 3", Object::Integer(6));

test_eval_ok!(bitwise_not, "~5", Object::Integer(-6));

test_eval_ok!(left_shift, "5 << 1", Object::Integer(10));

test_eval_ok!(right_shift, "5 >> 1", Object::Integer(2));

test_eval_ok!(
  bitwise_and_assignment,
  "let a = 5; a &= 3",
  Object::Integer(1)
);

test_eval_ok!(
  bitwise_or_assignment,
  "let a = 5; a |= 3",
  Object::Integer(7)
);

test_eval_ok!(
  bitwise_xor_assignment,
  "let a = 5; a ^= 3",
  Object::Integer(6)
);

test_eval_ok!(
  bitwise_left_shift_assignment,
  "let a = 5; a <<= 1",
  Object::Integer(10)
);

test_eval_ok!(
  bitwise_right_shift_assignment,
  "let a = 5; a >>= 1",
  Object::Integer(2)
);
