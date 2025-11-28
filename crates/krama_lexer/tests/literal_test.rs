use krama_core::token::TokenKind;
use krama_internal::{test_lexer, test_lexer_single};

test_lexer!(
  lex_integer_literals,
  "123 456 789",
  vec![
    TokenKind::Integer("123"),
    TokenKind::Integer("456"),
    TokenKind::Integer("789"),
  ]
);

test_lexer_single!(
  lex_string_literal,
  r#""hello world""#,
  TokenKind::String("hello world")
);

test_lexer_single!(
  lex_string_literal_with_escapes,
  r#""hello \"world\"""#,
  TokenKind::String(r#"hello \"world\""#)
);

test_lexer_single!(
  lex_unterminated_string_literal,
  r#""hello"#,
  TokenKind::Unknown
);
