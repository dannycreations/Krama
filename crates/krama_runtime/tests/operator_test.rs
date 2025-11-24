use krama_core::object::Object;
use krama_internal::test_eval;

test_eval!(should_int_add_op, "5 + 5", Object::Integer(10));

test_eval!(should_int_sub_op, "5 - 5", Object::Integer(0));

test_eval!(should_int_mul_op, "5 * 5", Object::Integer(25));

test_eval!(should_int_div_op, "5 / 5", Object::Integer(1));

test_eval!(should_int_mod_op, "5 % 2", Object::Integer(1));

test_eval!(should_int_exp_op, "2 ** 3", Object::Integer(8));

test_eval!(should_float_add_op, "5.0 + 5.0", Object::Float(10.0));

test_eval!(should_float_sub_op, "5.0 - 5.0", Object::Float(0.0));

test_eval!(should_float_mul_op, "5.0 * 5.0", Object::Float(25.0));

test_eval!(should_float_div_op, "5.0 / 5.0", Object::Float(1.0));

test_eval!(should_float_mod_op, "5.0 % 2.0", Object::Float(1.0));

test_eval!(should_float_exp_op, "2.0 ** 3.0", Object::Float(8.0));

test_eval!(should_prefix_inc_op, "let a = 5\n++a", Object::Integer(6));

test_eval!(should_postfix_inc_op, "let a = 5\na++", Object::Integer(5));

test_eval!(should_prefix_dec_op, "let a = 5\n--a", Object::Integer(4));

test_eval!(should_postfix_dec_op, "let a = 5\na--", Object::Integer(5));

test_eval!(
  should_plus_eq_assign_op,
  "let a = 5\na += 5",
  Object::Integer(10)
);

test_eval!(
  should_minus_eq_assign_op,
  "let a = 5\na -= 5",
  Object::Integer(0)
);

test_eval!(
  should_star_eq_assign_op,
  "let a = 5\na *= 5",
  Object::Integer(25)
);

test_eval!(
  should_slash_eq_assign_op,
  "let a = 5\na /= 5",
  Object::Integer(1)
);

test_eval!(
  should_percent_eq_assign_op,
  "let a = 5\na %= 2",
  Object::Integer(1)
);
