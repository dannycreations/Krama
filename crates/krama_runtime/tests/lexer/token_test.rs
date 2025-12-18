use krama_core::TokenKind;
use krama_runtime::test_lexer;

test_lexer!(
  lex_keyword_tokens,
  "const fn let if elif else match return break pub test true false import",
  vec![
    TokenKind::Const,
    TokenKind::Fn,
    TokenKind::Let,
    TokenKind::If,
    TokenKind::Elif,
    TokenKind::Else,
    TokenKind::Match,
    TokenKind::Return,
    TokenKind::Break,
    TokenKind::Pub,
    TokenKind::Test,
    TokenKind::True,
    TokenKind::False,
    TokenKind::Import,
  ]
);

test_lexer!(
  lex_single_character_tokens,
  "(){},+-*/[];",
  vec![
    TokenKind::LParen,
    TokenKind::RParen,
    TokenKind::LBrace,
    TokenKind::RBrace,
    TokenKind::Comma,
    TokenKind::Plus,
    TokenKind::Minus,
    TokenKind::Star,
    TokenKind::Slash,
    TokenKind::LBracket,
    TokenKind::RBracket,
    TokenKind::Semicolon
  ]
);

test_lexer!(
  lex_multi_character_tokens,
  "== != >= <= += => ..",
  vec![
    TokenKind::EqualEqual,
    TokenKind::BangEqual,
    TokenKind::GreaterThanEqual,
    TokenKind::LessThanEqual,
    TokenKind::PlusEqual,
    TokenKind::Arrow,
    TokenKind::DotDot,
  ]
);

test_lexer!(
  lex_single_line_comment,
  r#"
// This is a comment
123 // Number with a comment
// Another comment
"#,
  vec![TokenKind::Integer("123")]
);

test_lexer!(
  lex_multi_line_comment,
  r#"
/*
   This is a 
   multi-line comment
*/
123
"#,
  vec![TokenKind::Integer("123")]
);
