use krama_core::object::Object;
use krama_internal::test_eval;

test_eval!(should_eval_true, "true", Object::Boolean(true));

test_eval!(should_eval_false, "false", Object::Boolean(false));

test_eval!(should_eval_null, "null", Object::Null);

test_eval!(should_int_lit, "5", Object::Integer(5));

test_eval!(should_float_lit, "5.5", Object::Float(5.5));

test_eval!(
  should_int_with_separator,
  "1_000_000",
  Object::Integer(1000000)
);

test_eval!(
  should_float_with_separator,
  "1_000.50",
  Object::Float(1000.50)
);

test_eval!(should_scientific_notation, "1e5", Object::Float(100000.0));

test_eval!(
  should_scientific_notation_with_decimal,
  "1.5e2",
  Object::Float(150.0)
);

test_eval!(
  should_scientific_notation_with_negative_exponent,
  "1e-2",
  Object::Float(0.01)
);
