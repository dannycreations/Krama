use krama_core::ObjectKind;
use krama_runtime::test_eval_ok;

test_eval_ok!(
  match_expression_with_literal_pattern,
  "match (0) { 0 => 1, else => 2 }",
  ObjectKind::Integer(1)
);

test_eval_ok!(
  match_expression_with_else_clause,
  "match (1) { 0 => 1, else => 2 }",
  ObjectKind::Integer(2)
);

test_eval_ok!(
  match_expression_with_block_pattern,
  r#"
    match (1) {
      0 => 1,
      1 {
        const a = 1
        const b = 1
        a + b
      },
      else => 3
    }
  "#,
  ObjectKind::Integer(2)
);

test_eval_ok!(
  match_expression_with_multiple_patterns,
  "match (2) { 0 => 1, 1, 2, 3 => 2, else => 3 }",
  ObjectKind::Integer(2)
);

test_eval_ok!(
  match_expression_with_range_pattern,
  "match (5) { 0..10 => 1, else => 2 }",
  ObjectKind::Integer(1)
);

test_eval_ok!(
  match_expression_with_range_and_else,
  "match (11) { 0..10 => 1, else => 2 }",
  ObjectKind::Integer(2)
);

test_eval_ok!(
  match_expression_with_char_range_pattern,
  "match (\"b\") { \"a\"..\"z\" => 1, else => 2 }",
  ObjectKind::Integer(1)
);

test_eval_ok!(
  match_expression_with_multiple_char_ranges,
  "match (\"B\") { \"a\"..\"z\" => 1, \"A\"..\"Z\" => 2, else => 3 }",
  ObjectKind::Integer(2)
);

test_eval_ok!(
  match_expression_with_assignment,
  "const a = match (5) { 0..10 => 1, else => 2 }; a",
  ObjectKind::Integer(1)
);

test_eval_ok!(
  match_with_return_statement,
  r#"
    fn a() {
      match (true) {
        true { return 5 },
        false => 10
      }
    }
    a()
  "#,
  ObjectKind::Integer(5)
);

test_eval_ok!(
  match_with_break_statement,
  r#"
    let a = 0
    match (true) {
      true {
        a = 5
        break
        a = 10
      },
      false {}
    }
    a
  "#,
  ObjectKind::Integer(5)
);

test_eval_ok!(
  match_string_lexicographical_prefix,
  "match (\"car\") { \"cart\"..\"carz\" => 1, else => 2 }",
  ObjectKind::Integer(2)
);

test_eval_ok!(
  match_string_lexicographical_deeper,
  "match (\"cat\") { \"car\"..\"caz\" => 1, else => 2 }",
  ObjectKind::Integer(1)
);

test_eval_ok!(
  match_string_lexicographical_equal,
  "match (\"car\") { \"car\"..\"car\" => 1, else => 2 }",
  ObjectKind::Integer(1)
);
