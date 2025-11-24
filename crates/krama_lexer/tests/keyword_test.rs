use krama_core::token::TokenKind;
use krama_internal::test_lexer;

test_lexer!(
  should_lex_all_keywords,
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
