use krama_core::Object;
use krama_runtime::test_eval;

test_eval!(
  eval_for_in_array,
  r#"
    let result = 0
    let arr: u32[] = [1, 2, 3, 4, 5]
    for (x in arr) {
        result = result + x
    }
    result
    "#,
  Object::Integer(15)
);

test_eval!(
  eval_for_in_tuple,
  r#"
    let result = 0
    let tup = [10, 20, 30]
    for (x in tup) {
        result = result + x
    }
    result
    "#,
  Object::Integer(60)
);

test_eval!(
  eval_for_in_with_break,
  r#"
    let result = 0
    for (x in [1, 2, 3, 4, 5]) {
        if (x == 3) {
            break
        }
        result = result + x
    }
    result
    "#,
  Object::Integer(3)
);

test_eval!(
  eval_for_in_with_continue,
  r#"
    let result = 0
    for (x in [1, 2, 3, 4, 5]) {
        if (x % 2 == 0) {
            continue
        }
        result = result + x
    }
    result
    "#,
  Object::Integer(9)
);
