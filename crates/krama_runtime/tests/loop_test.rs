use krama_core::object::Object;
use krama_runtime::test_eval;

test_eval!(
  eval_while_loop,
  r#"
    let i = 0
    let result = 0
    while (i < 10) {
        result = result + i
        i = i + 1
    }
    result
    "#,
  Object::Integer(45)
);

test_eval!(
  eval_while_loop_with_break,
  r#"
    let i = 0
    while (i < 10) {
        if (i == 5) {
            break
        }
        i = i + 1
    }
    i
    "#,
  Object::Integer(5)
);

test_eval!(
  eval_while_loop_with_continue,
  r#"
    let i = 0
    let result = 0
    while (i < 10) {
        i = i + 1
        if (i % 2 == 0) {
            continue
        }
        result = result + i
    }
    result
    "#,
  Object::Integer(25)
);

test_eval!(
  eval_while_loop_with_return,
  r#"
    fn looper() {
        let i = 0
        while (i < 10) {
            if (i == 5) {
                return i
            }
            i = i + 1
        }
    }
    looper()
    "#,
  Object::Integer(5)
);
