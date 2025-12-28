use krama_core::Object;
use krama_runtime::test_eval_ok;

test_eval_ok!(
  iter_while,
  r#"
    let a = 0
    let b = 0
    while (a < 10) {
      b = b + a
      a = a + 1
    }
    b
  "#,
  Object::Integer(45)
);

test_eval_ok!(
  iter_while_with_break,
  r#"
    let a = 0
    while (a < 10) {
      if (a == 5) {
        break
      }
      a = a + 1
    }
    a
  "#,
  Object::Integer(5)
);

test_eval_ok!(
  iter_while_with_continue,
  r#"
    let a = 0
    let b = 0
    while (a < 10) {
      a = a + 1
      if (a % 2 == 0) {
        continue
      }
      b = b + a
    }
    b
  "#,
  Object::Integer(25)
);

test_eval_ok!(
  iter_while_with_return,
  r#"
    fn a() {
      let b = 0
      while (b < 10) {
        if (b == 5) {
          return b
        }
        b = b + 1
      }
    }
    a()
  "#,
  Object::Integer(5)
);
