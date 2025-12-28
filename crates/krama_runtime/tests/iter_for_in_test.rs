use krama_core::Object;
use krama_runtime::test_eval_ok;

test_eval_ok!(
  loop_for_in_array,
  r#"
    let a = 0
    const b: u32[] = [1, 2, 3, 4, 5]
    for (x in b) {
      a = a + x
    }
    a
  "#,
  Object::Integer(15)
);

test_eval_ok!(
  loop_for_in_tuple,
  r#"
    let a = 0
    const b = [10, 20, 30]
    for (x in b) {
      a = a + x
    }
    a
  "#,
  Object::Integer(60)
);

test_eval_ok!(
  loop_for_in_with_break,
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
  loop_for_in_with_continue,
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
  loop_for_in_with_range,
  r#"
    let a = 0
    for (x in 1..5) {
      a = a + x
    }
    a
  "#,
  Object::Integer(15)
);

test_eval_ok!(
  loop_for_in_destructure,
  r#"
    let a = 0
    const b = [[1, 2], [3, 4]]
    for ([x, y] in b) {
      a = a + x + y
    }
    a
  "#,
  Object::Integer(10)
);

test_eval_ok!(
  loop_for_in_object_kv,
  r#"
    let a = ""
    let sum = 0
    const o = { a: 1, b: 2 }
    for ([k, v] in o) {
      a = a + k
      sum = sum + v
    }
    a == "ab" && sum == 3
  "#,
  Object::Bool(true)
);

test_eval_ok!(
  loop_for_in_object_keys,
  r#"
    let a = ""
    const o = { a: 1, b: 2 }
    for (x in o) {
      a = a + x
    }
    a
  "#,
  Object::String("ab".into())
);

test_eval_ok!(
  loop_for_in_string,
  r#"
    let a = ""
    for (x in "hello") {
      a = a + x + "-"
    }
    a
  "#,
  Object::String("h-e-l-l-o-".into())
);
