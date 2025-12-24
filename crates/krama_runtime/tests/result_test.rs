use krama_core::{ErrorKind, ObjectKind};
use krama_runtime::{test_eval_err, test_eval_match, test_eval_ok};

// --- Basic Result Types ---

test_eval_match!(
  result_ok,
  "Ok(\"value\")",
  ObjectKind::Ok(val) if matches!(val, ObjectKind::String("value"))
);

test_eval_err!(result_err, "Err(\"error\")", ErrorKind::RuntimeError(_));

// --- Try Operator (?) ---

test_eval_match!(
  result_ok_as_value,
  "Ok(\"value\")?",
  ObjectKind::Ok(val) if matches!(val, ObjectKind::String("value"))
);

test_eval_err!(
  result_err_as_value,
  "Err(\"error\")?",
  ErrorKind::RuntimeError(_)
);

test_eval_ok!(result_non_try, "42?", ObjectKind::Integer(42));

// --- Error Propagation ---

test_eval_match!(
  result_ok_propagation,
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
  result_err_propagation,
  r#"
    fn fail() {
      Err("error")
      const a = true
    }

    fn main() {
      const x = fail()
      x + 1
    }

    main()
  "#,
  ErrorKind::RuntimeError(_)
);

test_eval_ok!(
  result_err_propagation_as_ignored_value,
  r#"
    fn fail() {
      Err("error")
      const a = true
    }

    fn main() {
      const _ = fail()?
    }

    main()
  "#,
  ObjectKind::Void
);

// --- Advanced Chaining & Nested Try ---

test_eval_match!(
  result_nested_try_chain,
  r#"
    fn first(x) { Ok(x + 1) }
    fn second(x) { Ok(x * 2) }
    fn main() {
      let val = first(1)?
      if (Ok(v) = val) {
        second(v)?
      } else {
        Err("unexpected")
      }
    }
    main()
  "#,
  ObjectKind::Ok(val) if matches!(val, ObjectKind::Integer(4))
);

test_eval_err!(
  result_chain_failure,
  r#"
    fn first() { Ok(1) }
    fn second() { Err("fail") }
    fn main() {
      let a = first()?
      let b = second()?
      a + b
    }
    main()
  "#,
  ErrorKind::RuntimeError(_)
);

// --- Pattern Matching with Results ---

test_eval_ok!(
  result_ok_if_pattern,
  r#"
    const a = Ok(42)
    if (Ok(b) = a) {
      b
    } elif (Err(b) = a) {
      b + 10
    } else {
      0
    }
  "#,
  ObjectKind::Integer(42)
);

test_eval_ok!(
  result_err_if_pattern,
  r#"
    const a = Err("error")?
    if (Err(e) = a) {
      e
    } else {
      "ok"
    }
  "#,
  ObjectKind::String("error")
);

test_eval_ok!(
  result_ok_match_pattern,
  r#"
    const a = Ok(100)
    match (a) {
      Ok(v) => v,
      Err(e) => 0,
    }
  "#,
  ObjectKind::Integer(100)
);

test_eval_ok!(
  result_err_match_pattern,
  r#"
    const a = Err(100)?
    match (a) {
      Ok(v) => v,
      Err(e) => 0,
    }
  "#,
  ObjectKind::Integer(0)
);

// --- Control Flow with Results ---

test_eval_ok!(
  result_ok_while_pattern,
  r#"
    let sum = 0
    let a = Ok(5)
    while (Ok(v) = a) {
      sum = sum + v
      if (v > 0) {
        a = Err(0)?
      } else {
        a = Ok(0)
      }
    }
    sum
  "#,
  ObjectKind::Integer(5)
);

test_eval_ok!(
  result_err_while_pattern,
  r#"
    let sum = 0
    let a = Err(5)?
    while (Err(v) = a) {
      sum = sum + v
      if (v > 0) {
        a = Ok(0)
      } else {
        a = Err(0)?
      }
    }
    sum
  "#,
  ObjectKind::Integer(5)
);
