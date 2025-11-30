use krama_core::object::Object;
use krama_runtime::test_eval;

test_eval!(eval_boolean_true, "true", Object::Boolean(true));

test_eval!(eval_boolean_false, "false", Object::Boolean(false));

test_eval!(eval_null_literal, "null", Object::Null);

test_eval!(eval_integer_literal, "5", Object::Integer(5));

test_eval!(eval_float_literal, "5.5", Object::Float(5.5));

test_eval!(
  eval_integer_with_separator,
  "1_000_000",
  Object::Integer(1000000)
);

test_eval!(
  eval_float_with_separator,
  "1_000.50",
  Object::Float(1000.50)
);

test_eval!(eval_scientific_notation, "1e5", Object::Float(100000.0));

test_eval!(
  eval_scientific_notation_with_decimal,
  "1.5e2",
  Object::Float(150.0)
);

test_eval!(
  eval_scientific_notation_with_negative_exponent,
  "1e-2",
  Object::Float(0.01)
);
