use krama_core::token::TokenKind;
use krama_internal::{test_lexer, test_lexer_single};

test_lexer!(
  should_lex_nums,
  "123 456 789",
  vec![
    TokenKind::Integer("123"),
    TokenKind::Integer("456"),
    TokenKind::Integer("789"),
  ]
);

test_lexer_single!(
  should_lex_string_lit,
  r#""hello world""#,
  TokenKind::String("hello world")
);

test_lexer_single!(
  should_lex_string_lit_escapes,
  r#""hello \"world\"""#,
  TokenKind::String(r#"hello \"world\""#)
);

test_lexer_single!(
  should_lex_unterminated_string_lit,
  r#""hello"#,
  TokenKind::Unknown
);
