use krama_core::ObjectKind;
use krama_runtime::test_eval_ok;

test_eval_ok!(literal_boolean_true, "true", ObjectKind::Boolean(true));

test_eval_ok!(literal_boolean_false, "false", ObjectKind::Boolean(false));

test_eval_ok!(literal_null, "null", ObjectKind::Null);

test_eval_ok!(literal_integer, "5", ObjectKind::Integer(5));

test_eval_ok!(literal_float, "5.5", ObjectKind::Float(5.5));

test_eval_ok!(
  literal_integer_with_separator,
  "1_000_000",
  ObjectKind::Integer(1000000)
);

test_eval_ok!(
  literal_float_with_separator,
  "1_000.50",
  ObjectKind::Float(1000.50)
);

test_eval_ok!(
  literal_scientific_notation,
  "1e5",
  ObjectKind::Float(100000.0)
);

test_eval_ok!(
  literal_scientific_notation_with_decimal,
  "1.5e2",
  ObjectKind::Float(150.0)
);

test_eval_ok!(
  literal_scientific_notation_with_negative_exponent,
  "1e-2",
  ObjectKind::Float(0.01)
);
