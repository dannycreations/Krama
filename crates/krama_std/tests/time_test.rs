use krama_core::Object;
use krama_runtime::test_eval_ok;

test_eval_ok!(
  eval_sleep,
  r#"
    import("std:time").sleep(10)
    "done"
  "#,
  Object::String("done")
);
