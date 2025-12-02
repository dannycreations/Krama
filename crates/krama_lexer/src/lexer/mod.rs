mod identifier;
mod number;
mod punctuator;
mod string;

use krama_core::{span::Span, token::Token};

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
            // Single-line comment
            self.advance_byte();
            self.advance_byte();
            while let Some(byte) = self.peek_byte() {
              if byte == b'\n' {
                break;
              }
              self.advance_byte();
            }
          } else if self.peek_byte_nth(1) == Some(b'*') {
            // Multi-line comment
            self.advance_byte();
            self.advance_byte();
            while let Some(byte) = self.advance_byte() {
              if byte == b'*' && self.peek_byte() == Some(b'/') {
                self.advance_byte();
                break;
              }
            }
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
    if self.position >= self.source.len() {
      return None;
    }

    let token = match self.peek_byte().unwrap() {
      b'"' => {
        self.advance_byte();
        self.string(start)
      }
      c if c.is_ascii_digit() => {
        self.advance_byte();
        self.number(start)
      }
      c if c.is_ascii_alphabetic() || c == b'_' => {
        self.advance_byte();
        self.identifier(start)
      }
      _ => self.punctuator(start),
    };

    Some(token)
  }
}
