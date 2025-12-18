use krama_core::Object;
use krama_runtime::test_eval_ok;

test_eval_ok!(
  eval_for_in_array,
  r#"
    let a = 0
    let b: u32[] = [1, 2, 3, 4, 5]
    for (x in b) {
      a = a + x
    }
    a
  "#,
  Object::Integer(15)
);

test_eval_ok!(
  eval_for_in_tuple,
  r#"
    let a = 0
    let b = [10, 20, 30]
    for (x in b) {
      a = a + x
    }
    a
  "#,
  Object::Integer(60)
);

test_eval_ok!(
  eval_for_in_with_break,
  r#"
    let a = 0
    for (x in [1, 2, 3, 4, 5]) {
      if (x == 3) {
        break
      }
      a = a + x
    }
    a
  "#,
  Object::Integer(3)
);

test_eval_ok!(
  eval_for_in_with_continue,
  r#"
    let a = 0
    for (x in [1, 2, 3, 4, 5]) {
      if (x % 2 == 0) {
        continue
      }
      a = a + x
    }
    a
  "#,
  Object::Integer(9)
);

test_eval_ok!(
  eval_for_in_with_range,
  r#"
    let a = 0
    for (x in 1..5) {
      a = a + x
    }
    a
  "#,
  Object::Integer(15)
);
