use krama_core::ObjectKind;
use krama_runtime::test_eval_ok;

test_eval_ok!(eval_bitwise_and, "5 & 3", ObjectKind::Integer(1));

test_eval_ok!(eval_bitwise_or, "5 | 3", ObjectKind::Integer(7));

test_eval_ok!(eval_bitwise_xor, "5 ^ 3", ObjectKind::Integer(6));

test_eval_ok!(eval_bitwise_not, "~5", ObjectKind::Integer(-6));

test_eval_ok!(eval_left_shift, "5 << 1", ObjectKind::Integer(10));

test_eval_ok!(eval_right_shift, "5 >> 1", ObjectKind::Integer(2));

test_eval_ok!(
  eval_bitwise_and_assignment,
  "let a = 5; a &= 3",
  ObjectKind::Integer(1)
);

test_eval_ok!(
  eval_bitwise_or_assignment,
  "let a = 5; a |= 3",
  ObjectKind::Integer(7)
);

test_eval_ok!(
  eval_bitwise_xor_assignment,
  "let a = 5; a ^= 3",
  ObjectKind::Integer(6)
);

test_eval_ok!(
  eval_left_shift_assignment,
  "let a = 5; a <<= 1",
  ObjectKind::Integer(10)
);

test_eval_ok!(
  eval_right_shift_assignment,
  "let a = 5; a >>= 1",
  ObjectKind::Integer(2)
);
