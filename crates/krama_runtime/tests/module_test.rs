use futures::future::FutureExt;
use krama_core::object::Object;
use krama_internal::{
  test_eval_async, test_eval_is_module, test_eval_is_native_function,
  test_eval_with_file,
};

test_eval_async!(
  should_cache_mods,
  r#"
        const assert1 = @import("std:assert")
        const assert2 = @import("std:assert")
        assert1 == assert2
    "#,
  Object::Boolean(true)
);

test_eval_is_module!(
  should_handle_aliasing_import,
  r#"
        const assert = @import("std:assert")
        assert
    "#,
  "assert"
);

test_eval_is_native_function!(
  should_handle_destructuring_import,
  r#"
        const { assert } = @import("std:assert")
        assert
    "#
);

test_eval_is_native_function!(
  should_handle_aliasing_destructuring_import,
  r#"
        const assert, { assertEqual as assert_equal } = @import("std:assert")
        assert
        assert_equal
    "#
);

test_eval_is_native_function!(
  should_eval_member_expr_on_mod,
  r#"
            const std = @import("std:assert")
            std.assert
        "#
);

test_eval_with_file!(
  should_import_file_mod,
  "math.kr",
  "pub fn add(a, b) { a + b }",
  r#"
        const math = @import("math.kr")
        math.add(1, 2)
    "#,
  Object::Integer(3)
);
