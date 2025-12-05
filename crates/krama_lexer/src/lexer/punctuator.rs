use krama_core::token::{Token, TokenKind};
use phf::{phf_map, Map};

use super::Lexer;

static PUNCTUATORS: Map<&'static [u8], TokenKind> = phf_map! {
    b"(" => TokenKind::LParen,
    b")" => TokenKind::RParen,
    b"{" => TokenKind::LBrace,
    b"}" => TokenKind::RBrace,
    b"[" => TokenKind::LBracket,
    b"]" => TokenKind::RBracket,
    b"," => TokenKind::Comma,
    b":" => TokenKind::Colon,
    b";" => TokenKind::Semicolon,
    b"~" => TokenKind::Tilde,
    b"%" => TokenKind::Percent,
    b"%=" => TokenKind::PercentEqual,
    b"/" => TokenKind::Slash,
    b"/=" => TokenKind::SlashEqual,
    b"!" => TokenKind::Bang,
    b"!=" => TokenKind::BangEqual,
    b"^" => TokenKind::Caret,
    b"^=" => TokenKind::CaretEqual,
    b"." => TokenKind::Dot,
    b".." => TokenKind::DotDot,
    b"+" => TokenKind::Plus,
    b"++" => TokenKind::PlusPlus,
    b"+=" => TokenKind::PlusEqual,
    b"-" => TokenKind::Minus,
    b"--" => TokenKind::MinusMinus,
    b"-=" => TokenKind::MinusEqual,
    b"*" => TokenKind::Star,
    b"**" => TokenKind::StarStar,
    b"*=" => TokenKind::StarEqual,
    b"&" => TokenKind::Ampersand,
    b"&&" => TokenKind::AmpersandAmpersand,
    b"&=" => TokenKind::AmpersandEqual,
    b"|" => TokenKind::Pipe,
    b"||" => TokenKind::PipePipe,
    b"|=" => TokenKind::PipeEqual,
    b"=" => TokenKind::Equal,
    b"=>" => TokenKind::Arrow,
    b"==" => TokenKind::EqualEqual,
    b">" => TokenKind::GreaterThan,
    b">>" => TokenKind::GreaterGreater,
    b">>=" => TokenKind::GreaterGreaterEqual,
    b">=" => TokenKind::GreaterThanEqual,
    b"<" => TokenKind::LessThan,
    b"<<" => TokenKind::LessLess,
    b"<<=" => TokenKind::LessLessEqual,
    b"<=" => TokenKind::LessThanEqual,
};

impl<'a> Lexer<'a> {
  pub(super) fn punctuator(&mut self, start: usize) -> Token<'a> {
    // Check for 3-byte punctuator
    if let Some(bytes) = self.source.get(start..start + 3) {
      if let Some(&kind) = PUNCTUATORS.get(bytes) {
        self.position = start + 3;
        return Token::new(kind, self.span(start));
      }
    }

    // Check for 2-byte punctuator
    if let Some(bytes) = self.source.get(start..start + 2) {
      if let Some(&kind) = PUNCTUATORS.get(bytes) {
        self.position = start + 2;
        return Token::new(kind, self.span(start));
      }
    }

    // Check for 1-byte punctuator
    if let Some(bytes) = self.source.get(start..start + 1) {
      if let Some(&kind) = PUNCTUATORS.get(bytes) {
        self.position = start + 1;
        return Token::new(kind, self.span(start));
      }
    }

    // If no match, it's an unknown token.
    self.position = start + 1;
    Token::new(TokenKind::Unknown, self.span(start))
  }
}
