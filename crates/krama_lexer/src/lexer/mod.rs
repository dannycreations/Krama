mod identifier;
mod number;
mod string;

use krama_core::{
  span::Span,
  token::{Token, TokenKind},
};

macro_rules! token {
  ($lexer:ident, $start:expr, $kind:expr) => {
    Token::new($kind, $lexer.span($start))
  };

  ($lexer:ident, $start:expr, $one_char:expr, $next_char:expr, $two_chars:expr) => {{
    let kind = if $lexer.advance_if_byte($next_char) {
      $two_chars
    } else {
      $one_char
    };
    token!($lexer, $start, kind)
  }};
}

#[derive(Clone)]
pub struct Lexer<'a> {
  source: &'a [u8],
  file: Option<&'a str>,
  position: usize,
}

impl<'a> Lexer<'a> {
  pub fn new(source: &'a str, file: Option<&'a str>) -> Self {
    Self {
      source: source.as_bytes(),
      file,
      position: 0,
    }
  }

  pub fn source_len(&self) -> usize {
    self.source.len()
  }

  pub(super) fn peek_byte(&self) -> Option<u8> {
    self.source.get(self.position).copied()
  }

  pub(super) fn peek_byte_nth(&self, n: usize) -> Option<u8> {
    self.source.get(self.position + n).copied()
  }

  pub(super) fn advance_byte(&mut self) -> Option<u8> {
    let byte = self.source.get(self.position).copied();
    self.position += 1;
    byte
  }

  pub(super) fn advance_if_byte(&mut self, expected: u8) -> bool {
    if self.peek_byte() == Some(expected) {
      self.position += 1;
      return true;
    }
    false
  }

  pub(super) fn span(&self, start: usize) -> Span<'a> {
    Span::new(
      start,
      self.position,
      Some(self.slice(0, self.source_len())),
      self.file,
    )
  }

  pub(super) fn slice(&self, start: usize, end: usize) -> &'a str {
    std::str::from_utf8(&self.source[start..end]).unwrap()
  }

  fn skip_trivia(&mut self) {
    loop {
      match self.peek_byte() {
        Some(c) if c.is_ascii_whitespace() => {
          self.advance_byte();
        }
        Some(b'/') => {
          if self.peek_byte_nth(1) == Some(b'/') {
            while self.peek_byte().is_some() && self.peek_byte() != Some(b'\n')
            {
              self.advance_byte();
            }
          } else if self.peek_byte_nth(1) == Some(b'*') {
            self.advance_byte();
            self.advance_byte();
            while self.peek_byte().is_some()
              && (self.peek_byte() != Some(b'*')
                || self.peek_byte_nth(1) != Some(b'/'))
            {
              self.advance_byte();
            }
            self.advance_byte();
            self.advance_byte();
          } else {
            break;
          }
        }
        _ => break,
      }
    }
  }
}

impl<'a> Iterator for Lexer<'a> {
  type Item = Token<'a>;

  fn next(&mut self) -> Option<Self::Item> {
    self.skip_trivia();

    let start = self.position;
    let byte = self.advance_byte()?;

    let token = match byte {
      b'(' => token!(self, start, TokenKind::LParen),
      b')' => token!(self, start, TokenKind::RParen),
      b'{' => token!(self, start, TokenKind::LBrace),
      b'}' => token!(self, start, TokenKind::RBrace),
      b'[' => token!(self, start, TokenKind::LBracket),
      b']' => token!(self, start, TokenKind::RBracket),
      b',' => token!(self, start, TokenKind::Comma),
      b':' => token!(self, start, TokenKind::Colon),
      b';' => token!(self, start, TokenKind::Semicolon),
      b'~' => token!(self, start, TokenKind::Tilde),
      b'%' => token!(
        self,
        start,
        TokenKind::Percent,
        b'=',
        TokenKind::PercentEqual
      ),
      b'/' => {
        token!(self, start, TokenKind::Slash, b'=', TokenKind::SlashEqual)
      }
      b'!' => token!(self, start, TokenKind::Bang, b'=', TokenKind::BangEqual),
      b'^' => {
        token!(self, start, TokenKind::Caret, b'=', TokenKind::CaretEqual)
      }
      b'.' => {
        if self.advance_if_byte(b'.') {
          token!(self, start, TokenKind::DotDot)
        } else {
          token!(self, start, TokenKind::Dot)
        }
      }
      b'+' => {
        if self.advance_if_byte(b'+') {
          token!(self, start, TokenKind::PlusPlus)
        } else if self.advance_if_byte(b'=') {
          token!(self, start, TokenKind::PlusEqual)
        } else {
          token!(self, start, TokenKind::Plus)
        }
      }
      b'-' => {
        if self.advance_if_byte(b'-') {
          token!(self, start, TokenKind::MinusMinus)
        } else if self.advance_if_byte(b'=') {
          token!(self, start, TokenKind::MinusEqual)
        } else {
          token!(self, start, TokenKind::Minus)
        }
      }
      b'*' => {
        if self.advance_if_byte(b'*') {
          token!(self, start, TokenKind::StarStar)
        } else if self.advance_if_byte(b'=') {
          token!(self, start, TokenKind::StarEqual)
        } else {
          token!(self, start, TokenKind::Star)
        }
      }
      b'&' => {
        if self.advance_if_byte(b'&') {
          token!(self, start, TokenKind::AmpersandAmpersand)
        } else if self.advance_if_byte(b'=') {
          token!(self, start, TokenKind::AmpersandEqual)
        } else {
          token!(self, start, TokenKind::Ampersand)
        }
      }
      b'|' => {
        if self.advance_if_byte(b'|') {
          token!(self, start, TokenKind::PipePipe)
        } else if self.advance_if_byte(b'=') {
          token!(self, start, TokenKind::PipeEqual)
        } else {
          token!(self, start, TokenKind::Pipe)
        }
      }
      b'=' => {
        if self.advance_if_byte(b'>') {
          token!(self, start, TokenKind::Arrow)
        } else if self.advance_if_byte(b'=') {
          token!(self, start, TokenKind::EqualEqual)
        } else {
          token!(self, start, TokenKind::Equal)
        }
      }
      b'>' => {
        if self.advance_if_byte(b'>') {
          if self.advance_if_byte(b'=') {
            token!(self, start, TokenKind::GreaterGreaterEqual)
          } else {
            token!(self, start, TokenKind::GreaterGreater)
          }
        } else if self.advance_if_byte(b'=') {
          token!(self, start, TokenKind::GreaterThanEqual)
        } else {
          token!(self, start, TokenKind::GreaterThan)
        }
      }
      b'<' => {
        if self.advance_if_byte(b'<') {
          if self.advance_if_byte(b'=') {
            token!(self, start, TokenKind::LessLessEqual)
          } else {
            token!(self, start, TokenKind::LessLess)
          }
        } else if self.advance_if_byte(b'=') {
          token!(self, start, TokenKind::LessThanEqual)
        } else {
          token!(self, start, TokenKind::LessThan)
        }
      }
      b'"' => self.string(start),
      c if c.is_ascii_digit() => self.number(start),
      c if c.is_ascii_alphabetic() || c == b'_' => self.identifier(start),
      _ => token!(self, start, TokenKind::Unknown),
    };

    Some(token)
  }
}
