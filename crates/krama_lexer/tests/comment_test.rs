use krama_core::token::TokenKind;
use krama_internal::test_lexer;

test_lexer!(
  should_lex_single_line_comment_and_ignore,
  r#"
// this is a comment
123 // number with a comment
// another comment
"#,
  vec![
    TokenKind::Newline,
    TokenKind::Newline,
    TokenKind::Integer("123"),
    TokenKind::Newline,
    TokenKind::Newline,
  ]
);
