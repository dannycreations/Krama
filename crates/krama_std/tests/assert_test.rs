use krama_core::error::ErrorKind;
use krama_core::object::Object;
use krama_internal::{test_eval, test_eval_error};

test_eval!(
  should_assert_mod,
  r#"
        const { assert } = @import("std:assert")
        assert(true)
    "#,
  Object::Void
);

test_eval!(
  should_assert_eq_mod,
  r#"
        const { assertEqual } = @import("std:assert")
        assertEqual(1, 1)
    "#,
  Object::Void
);

test_eval_error!(
  should_assert_failing,
  r#"
        const { assert } = @import("std:assert")
        assert(false)
    "#,
  ErrorKind::RuntimeError(_)
);

test_eval_error!(
  should_assert_eq_failing,
  r#"
        const { assertEqual } = @import("std:assert")
        assertEqual(1, 2)
    "#,
  ErrorKind::RuntimeError(_)
);
