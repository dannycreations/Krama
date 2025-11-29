use krama_core::{error::ErrorKind, span::Span};
use krama_internal::test_parser_error;

macro_rules! test_keyword_as_identifier_error {
  ($name:ident, $keyword:expr) => {
    test_parser_error!($name, $keyword, |error: (ErrorKind, Span)| {
      assert!(matches!(error.0, ErrorKind::SyntaxError(_)));
    });
  };
}

test_keyword_as_identifier_error!(
  parse_const_keyword_as_identifier,
  "const const = 5"
);
test_keyword_as_identifier_error!(
  parse_fn_keyword_as_identifier,
  "const fn = 5"
);
test_keyword_as_identifier_error!(
  parse_pub_keyword_as_identifier,
  "const pub = 5"
);
test_keyword_as_identifier_error!(
  parse_let_keyword_as_identifier,
  "const let = 5"
);
test_keyword_as_identifier_error!(
  parse_if_keyword_as_identifier,
  "const if = 5"
);
test_keyword_as_identifier_error!(
  parse_elif_keyword_as_identifier,
  "const elif = 5"
);
test_keyword_as_identifier_error!(
  parse_else_keyword_as_identifier,
  "const else = 5"
);
test_keyword_as_identifier_error!(
  parse_match_keyword_as_identifier,
  "const match = 5"
);
test_keyword_as_identifier_error!(
  parse_while_keyword_as_identifier,
  "const while = 5"
);
test_keyword_as_identifier_error!(
  parse_return_keyword_as_identifier,
  "const return = 5"
);
test_keyword_as_identifier_error!(
  parse_break_keyword_as_identifier,
  "const break = 5"
);
test_keyword_as_identifier_error!(
  parse_continue_keyword_as_identifier,
  "const continue = 5"
);
test_keyword_as_identifier_error!(
  parse_keyword_as_identifier,
  "const test = 5"
);
test_keyword_as_identifier_error!(
  parse_true_keyword_as_identifier,
  "const true = 5"
);
test_keyword_as_identifier_error!(
  parse_false_keyword_as_identifier,
  "const false = 5"
);
test_keyword_as_identifier_error!(
  parse_import_keyword_as_identifier,
  "const import = 5"
);
test_keyword_as_identifier_error!(
  parse_as_keyword_as_identifier,
  "const as = 5"
);
test_keyword_as_identifier_error!(
  parse_null_keyword_as_identifier,
  "const null = 5"
);
