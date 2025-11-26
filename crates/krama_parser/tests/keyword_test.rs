use krama_core::error::{Error, ErrorKind};
use krama_internal::test_parser_error;

macro_rules! test_keyword_as_identifier {
  ($name:ident, $keyword:expr) => {
    test_parser_error!($name, $keyword, |error: Error| {
      assert!(matches!(error.kind, ErrorKind::SyntaxError(_)));
    });
  };
}

test_keyword_as_identifier!(
  should_fail_when_const_is_used_as_identifier,
  "const const = 5"
);
test_keyword_as_identifier!(
  should_fail_when_fn_is_used_as_identifier,
  "const fn = 5"
);
test_keyword_as_identifier!(
  should_fail_when_pub_is_used_as_identifier,
  "const pub = 5"
);
test_keyword_as_identifier!(
  should_fail_when_let_is_used_as_identifier,
  "const let = 5"
);
test_keyword_as_identifier!(
  should_fail_when_if_is_used_as_identifier,
  "const if = 5"
);
test_keyword_as_identifier!(
  should_fail_when_elif_is_used_as_identifier,
  "const elif = 5"
);
test_keyword_as_identifier!(
  should_fail_when_else_is_used_as_identifier,
  "const else = 5"
);
test_keyword_as_identifier!(
  should_fail_when_match_is_used_as_identifier,
  "const match = 5"
);
test_keyword_as_identifier!(
  should_fail_when_while_is_used_as_identifier,
  "const while = 5"
);
test_keyword_as_identifier!(
  should_fail_when_return_is_used_as_identifier,
  "const return = 5"
);
test_keyword_as_identifier!(
  should_fail_when_break_is_used_as_identifier,
  "const break = 5"
);
test_keyword_as_identifier!(
  should_fail_when_continue_is_used_as_identifier,
  "const continue = 5"
);
test_keyword_as_identifier!(
  should_fail_when_test_is_used_as_identifier,
  "const test = 5"
);
test_keyword_as_identifier!(
  should_fail_when_true_is_used_as_identifier,
  "const true = 5"
);
test_keyword_as_identifier!(
  should_fail_when_false_is_used_as_identifier,
  "const false = 5"
);
test_keyword_as_identifier!(
  should_fail_when_import_is_used_as_identifier,
  "const import = 5"
);
test_keyword_as_identifier!(
  should_fail_when_as_is_used_as_identifier,
  "const as = 5"
);
test_keyword_as_identifier!(
  should_fail_when_null_is_used_as_identifier,
  "const null = 5"
);
