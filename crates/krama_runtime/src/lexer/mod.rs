mod identifier;
mod number;
mod punctuator;
mod string;

use krama_core::{Span, Token};

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

  #[inline(always)]
  pub fn source_len(&self) -> usize {
    self.source.len()
  }

  #[inline(always)]
  pub fn peek_byte(&self) -> Option<u8> {
    self.source.get(self.position).copied()
  }

  #[inline(always)]
  pub fn peek_byte_nth(&self, n: usize) -> Option<u8> {
    self.source.get(self.position + n).copied()
  }

  #[inline(always)]
  pub fn advance_byte(&mut self) -> Option<u8> {
    let byte = self.source.get(self.position).copied();
    if byte.is_some() {
      self.position += 1;
    }
    byte
  }

  pub fn span(&self, start: usize) -> Span<'a> {
    Span::new(
      start,
      self.position,
      Some(self.slice(0, self.source_len())),
      self.file,
    )
  }

  #[inline(always)]
  pub fn slice(&self, start: usize, end: usize) -> &'a str {
    // SAFETY: Input source is guaranteed to be valid UTF-8 str
    unsafe { std::str::from_utf8_unchecked(&self.source[start..end]) }
  }

  fn skip_trivia(&mut self) {
    while let Some(c) = self.peek_byte() {
      match c {
        b' ' | b'\r' | b'\t' | b'\n' => {
          self.position += 1;
        }
        b'/' => match self.peek_byte_nth(1) {
          Some(b'/') => {
            self.position += 2;
            while let Some(byte) = self.peek_byte() {
              if byte == b'\n' {
                break;
              }
              self.position += 1;
            }
          }
          Some(b'*') => {
            self.position += 2;
            while let Some(byte) = self.advance_byte() {
              if byte == b'*' && self.peek_byte() == Some(b'/') {
                self.position += 1;
                break;
              }
            }
          }
          _ => break,
        },
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
    let byte = self.peek_byte()?;

    let token = match byte {
      b'"' => {
        self.position += 1;
        self.string(start)
      }
      b'0'..=b'9' => {
        self.position += 1;
        self.number(start)
      }
      b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
        self.position += 1;
        self.identifier(start)
      }
      _ => self.punctuator(start),
    };

    Some(token)
  }
}
