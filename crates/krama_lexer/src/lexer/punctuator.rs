use krama_core::token::{Token, TokenKind};
use phf::{phf_map, Map};

use super::Lexer;

static PUNCTUATORS: Map<&'static str, TokenKind> = phf_map! {
    "(" => TokenKind::LParen,
    ")" => TokenKind::RParen,
    "{" => TokenKind::LBrace,
    "}" => TokenKind::RBrace,
    "[" => TokenKind::LBracket,
    "]" => TokenKind::RBracket,
    "," => TokenKind::Comma,
    ":" => TokenKind::Colon,
    ";" => TokenKind::Semicolon,
    "~" => TokenKind::Tilde,
    "%" => TokenKind::Percent,
    "%=" => TokenKind::PercentEqual,
    "/" => TokenKind::Slash,
    "/=" => TokenKind::SlashEqual,
    "!" => TokenKind::Bang,
    "!=" => TokenKind::BangEqual,
    "^" => TokenKind::Caret,
    "^=" => TokenKind::CaretEqual,
    "." => TokenKind::Dot,
    ".." => TokenKind::DotDot,
    "+" => TokenKind::Plus,
    "++" => TokenKind::PlusPlus,
    "+=" => TokenKind::PlusEqual,
    "-" => TokenKind::Minus,
    "--" => TokenKind::MinusMinus,
    "-=" => TokenKind::MinusEqual,
    "*" => TokenKind::Star,
    "**" => TokenKind::StarStar,
    "*=" => TokenKind::StarEqual,
    "&" => TokenKind::Ampersand,
    "&&" => TokenKind::AmpersandAmpersand,
    "&=" => TokenKind::AmpersandEqual,
    "|" => TokenKind::Pipe,
    "||" => TokenKind::PipePipe,
    "|=" => TokenKind::PipeEqual,
    "=" => TokenKind::Equal,
    "=>" => TokenKind::Arrow,
    "==" => TokenKind::EqualEqual,
    ">" => TokenKind::GreaterThan,
    ">>" => TokenKind::GreaterGreater,
    ">>=" => TokenKind::GreaterGreaterEqual,
    ">=" => TokenKind::GreaterThanEqual,
    "<" => TokenKind::LessThan,
    "<<" => TokenKind::LessLess,
    "<<=" => TokenKind::LessLessEqual,
    "<=" => TokenKind::LessThanEqual,
};

impl<'a> Lexer<'a> {
  pub(super) fn punctuator(&mut self, start: usize) -> Token<'a> {
    let mut end = self.position;
    let mut last_match: Option<(TokenKind, usize)> = None;

    while end <= self.source_len() {
      let slice = self.slice(start, end);
      if let Some(&kind) = PUNCTUATORS.get(slice) {
        last_match = Some((kind, end));
      } else if last_match.is_some() {
        // We've gone past the longest possible match
        break;
      }
      end += 1;
    }

    if let Some((kind, end_pos)) = last_match {
      self.position = end_pos;
      Token::new(kind, self.span(start))
    } else {
      // If no match was found, it's an unknown character.
      // We advance one byte and report it.
      self.position = start + 1;
      Token::new(TokenKind::Unknown, self.span(start))
    }
  }
}
