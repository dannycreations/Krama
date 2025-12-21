use krama_core::{ErrorKind, Object};
use krama_runtime::{test_eval_err, test_eval_ok};

test_eval_ok!(
  eval_assert_module,
  r#"
    const { assert } = import("std:assert")
    assert(true)
  "#,
  Object::Void
);

test_eval_ok!(
  eval_assert_equal_module,
  r#"
    const { assertEqual } = import("std:assert")
    assertEqual(1, 1)
  "#,
  Object::Void
);

test_eval_err!(
  eval_failing_assert,
  r#"
    const { assert } = import("std:assert")
    assert(false)
  "#,
  ErrorKind::RuntimeError(_)
);

test_eval_err!(
  eval_failing_assert_equal,
  r#"
    const { assertEqual } = import("std:assert")
    assertEqual(1, 2)
  "#,
  ErrorKind::RuntimeError(_)
);
