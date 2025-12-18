use krama_core::Object;
use krama_runtime::{
  test_eval, test_eval_is_module, test_eval_is_native_function,
  test_eval_with_file,
};

test_eval!(
  eval_module_caching,
  r#"
        const assert1 = import("std:assert")
        const assert2 = import("std:assert")
        assert1 == assert2
    "#,
  Object::Boolean(true)
);

test_eval_is_module!(
  eval_module_import_with_aliasing,
  r#"
        const assert = import("std:assert")
        assert
    "#,
  Some("assert")
);

test_eval_is_native_function!(
  eval_module_import_with_destructuring,
  r#"
        const { assertEqual } = import("std:assert")
        assertEqual
    "#
);

test_eval_is_native_function!(
  eval_module_import_with_aliasing_and_destructuring,
  r#"
        const assert, { assertEqual as assert_equal } = import("std:assert")
        assert_equal
    "#
);

test_eval_is_native_function!(
  eval_member_expression_on_module,
  r#"
            const std = import("std:assert")
            std.assert
        "#
);

test_eval_with_file!(
  eval_file_module_import,
  "math.kr",
  "pub fn add(a, b) { a + b }",
  r#"
        const math = import("./math.kr")
        math.add(1, 2)
    "#,
  Object::Integer(3)
);
