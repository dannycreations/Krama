use krama_core::ObjectKind;
use krama_runtime::test_eval_ok;

test_eval_ok!(bitwise_and, "5 & 3", ObjectKind::Integer(1));

test_eval_ok!(bitwise_or, "5 | 3", ObjectKind::Integer(7));

test_eval_ok!(bitwise_xor, "5 ^ 3", ObjectKind::Integer(6));

test_eval_ok!(bitwise_not, "~5", ObjectKind::Integer(-6));

test_eval_ok!(left_shift, "5 << 1", ObjectKind::Integer(10));

test_eval_ok!(right_shift, "5 >> 1", ObjectKind::Integer(2));

test_eval_ok!(
  bitwise_and_assignment,
  "let a = 5; a &= 3",
  ObjectKind::Integer(1)
);

test_eval_ok!(
  bitwise_or_assignment,
  "let a = 5; a |= 3",
  ObjectKind::Integer(7)
);

test_eval_ok!(
  bitwise_xor_assignment,
  "let a = 5; a ^= 3",
  ObjectKind::Integer(6)
);

test_eval_ok!(
  bitwise_left_shift_assignment,
  "let a = 5; a <<= 1",
  ObjectKind::Integer(10)
);

test_eval_ok!(
  bitwise_right_shift_assignment,
  "let a = 5; a >>= 1",
  ObjectKind::Integer(2)
);
