use krama_core::token::TokenKind;
use krama_internal::test_lexer;

test_lexer!(
  should_lex_single_char_tokens,
  "(){},+-*/@[];",
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
    TokenKind::At,
    TokenKind::LBracket,
    TokenKind::RBracket,
    TokenKind::Semicolon
  ]
);

test_lexer!(
  should_lex_multi_char_tokens,
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
  should_lex_at_import_token,
  "@import",
  vec![TokenKind::At, TokenKind::Import]
);
