use krama_core::object::Object;
use krama_runtime::test_eval;

test_eval!(
  eval_match_expression_with_literal_pattern,
  "match (0) { 0 => 1, else => 2 }",
  Object::Integer(1)
);

test_eval!(
  eval_match_expression_with_else_clause,
  "match (1) { 0 => 1, else => 2 }",
  Object::Integer(2)
);

test_eval!(
  eval_match_expression_with_block_pattern,
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
  Object::Integer(2)
);

test_eval!(
  eval_match_expression_with_multiple_patterns,
  "match (2) { 0 => 1, 1, 2, 3 => 2, else => 3 }",
  Object::Integer(2)
);

test_eval!(
  eval_match_expression_with_range_pattern,
  "match (5) { 0..10 => 1, else => 2 }",
  Object::Integer(1)
);

test_eval!(
  eval_match_expression_with_range_and_else,
  "match (11) { 0..10 => 1, else => 2 }",
  Object::Integer(2)
);

test_eval!(
  eval_match_expression_with_char_range_pattern,
  r#"match ("b") { "a".."z" => 1, else => 2 }"#,
  Object::Integer(1)
);

test_eval!(
  eval_match_expression_with_multiple_char_ranges,
  r#"match ("B") { "a".."z" => 1, "A".."Z" => 2, else => 3 }"#,
  Object::Integer(2)
);

test_eval!(
  eval_match_expression_with_assignment,
  "const x = match (5) { 0..10 => 1, else => 2 }\nx",
  Object::Integer(1)
);

test_eval!(
  eval_match_with_return_statement,
  r#"
        fn my_test() {
            match (true) {
                true { return 5 },
                false => 10
            }
        }
        my_test()
    "#,
  Object::Integer(5)
);

test_eval!(
  eval_match_with_break_statement,
  r#"
        let x = 0
        match (true) {
            true {
                x = 5
                break
                x = 10
            },
            false {}
        }
        x
    "#,
  Object::Integer(5)
);
