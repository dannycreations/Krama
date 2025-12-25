use krama_core::ObjectKind;
use krama_runtime::test_eval_ok;

test_eval_ok!(
  time_sleep,
  r#"
    import("std:time").sleep(10)
    "done"
  "#,
  ObjectKind::String("done".into())
);
