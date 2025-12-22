use krama_core::{ErrorKind, ObjectKind};
use krama_runtime::{test_eval_err, test_eval_match, test_eval_ok};

test_eval_match!(
  ok_result,
  "Ok(\"value\")",
  ObjectKind::Ok(val) if matches!(val, ObjectKind::String("value"))
);

test_eval_err!(err_result, "Err(\"error\")", ErrorKind::RuntimeError(_));

test_eval_match!(
  ok_result_as_value,
  "Ok(\"value\")?",
  ObjectKind::Ok(val) if matches!(val, ObjectKind::String("value"))
);

test_eval_err!(
  err_result_as_value,
  "Err(\"error\")?",
  ErrorKind::RuntimeError(_)
);

test_eval_ok!(non_result_try, "42?", ObjectKind::Integer(42));

test_eval_match!(
  ok_propagation,
  r#"
    fn success() {
      Ok("value")
    }

    fn main() {
      success()
    }

    main()
  "#,
  ObjectKind::Ok(val) if matches!(val, ObjectKind::String("value"))
);

test_eval_err!(
  err_propagation,
  r#"
    fn fail() {
      Err("error")
      const a = true
    }

    fn main() {
      const x = fail();
      x + 1
    }

    main()
  "#,
  ErrorKind::RuntimeError(_)
);

test_eval_ok!(
  err_propagation_as_ignored_value,
  r#"
    fn fail() {
      Err("error")
      const a = true
    }

    fn main() {
      const _ = fail()?;
    }

    main()
  "#,
  ObjectKind::Void
);
