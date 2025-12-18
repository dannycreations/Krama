use krama_core::{ErrorKind, Object};
use krama_runtime::{test_eval_err, test_eval_ok};

test_eval_ok!(test_ok_result, "Ok(42)?", Object::Integer(42));

test_eval_err!(
  test_err_result,
  "Err(\"oops\")?",
  ErrorKind::RuntimeError(_)
);

test_eval_err!(test_non_result_try, "42?", ErrorKind::TypeError(_));

test_eval_ok!(
  test_result_propagation_ok,
  r#"
    fn divide(a, b) {
      if (b == 0) {
        return Err("division by zero");
      };
      Ok(a / b)
    }

    fn calculate() {
      let x = divide(10, 2)?;
      x + 5
    }

    calculate()
  "#,
  Object::Integer(10)
);

test_eval_err!(
  test_result_propagation_err,
  r#"
    fn fail() {
      Err("explicit failure")
    }

    fn calculate() {
      let x = fail()?;
      x + 1
    }

    calculate()
  "#,
  ErrorKind::RuntimeError(_)
);
