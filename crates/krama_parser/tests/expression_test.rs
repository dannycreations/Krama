use krama_internal::test_parser;

test_parser!(should_parse_unary_exprs, "!true", 1);

test_parser!(should_parse_binary_exprs, "1 + 2", 1);

test_parser!(should_parse_call_expr, "add(1, 2)", 1);

test_parser!(should_parse_match_expr, "match (x) { 1 => 2 }", 1);

test_parser!(should_parse_import_expr, r#"@import("std:fs")"#, 1);

test_parser!(
  should_parse_destructuring_import_expr,
  r#"const { a, b } = @import("std:fs")"#,
  1
);
