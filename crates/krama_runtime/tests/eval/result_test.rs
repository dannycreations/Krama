use krama_core::{ErrorKind, Object};
use krama_runtime::{test_eval, test_eval_error};

test_eval!(test_ok_result, "Ok(42)?", Object::Integer(42));

test_eval_error!(
  test_err_result,
  "Err(\"oops\")?",
  ErrorKind::RuntimeError(_)
);

test_eval_error!(test_non_result_try, "42?", ErrorKind::TypeError(_));

test_eval!(
  test_result_propagation_ok,
  "
    fn divide(a, b) {
      if (b == 0) {
        return Err(\"division by zero\");
      };
      Ok(a / b)
    }

    fn calculate() {
      let x = divide(10, 2)?;
      x + 5
    }

    calculate()
  ",
  Object::Integer(10)
);

test_eval_error!(
  test_result_propagation_err,
  "
    fn fail() {
      Err(\"explicit failure\")
    }

    fn calculate() {
      let x = fail()?;
      x + 1
    }

    calculate()
  ",
  ErrorKind::RuntimeError(_)
);
