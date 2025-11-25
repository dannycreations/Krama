mod identifier;
mod number;
mod string;

use krama_core::span::Span;
use krama_core::token::Token;
use krama_core::token::TokenKind;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Clone)]
pub struct Lexer<'a> {
  pub(super) input: Peekable<Chars<'a>>,
  pub(super) position: usize,
  input_str: &'a str,
}

impl<'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Self {
    Self {
      input_str: input,
      input: input.chars().peekable(),
      position: 0,
    }
  }

  pub fn input_len(&self) -> usize {
    self.input_str.len()
  }

  pub(super) fn peek(&mut self) -> Option<char> {
    self.input.peek().copied()
  }

  pub(super) fn advance(&mut self) -> Option<char> {
    let char = self.input.next()?;
    self.position += char.len_utf8();
    Some(char)
  }

  pub(super) fn advance_if(&mut self, expected: char) -> bool {
    if self.peek() == Some(expected) {
      self.advance();
      true
    } else {
      false
    }
  }

  pub(super) fn span(&self, start: usize) -> Span {
    Span::new(start, self.position)
  }

  fn skip_whitespace(&mut self) {
    loop {
      match self.peek() {
        Some('/') if self.input.clone().nth(1) == Some('/') => {
          // It's a comment. Consume the two slashes.
          self.advance();
          self.advance();
          // Now consume until newline.
          while let Some(c) = self.peek() {
            if c == '\n' {
              break;
            }
            self.advance();
          }
        }
        Some(c) if c.is_whitespace() && c != '\n' => {
          self.advance();
        }
        _ => {
          break;
        }
      }
    }
  }
}

impl<'a> Iterator for Lexer<'a> {
  type Item = Token<'a>;

  fn next(&mut self) -> Option<Self::Item> {
    self.skip_whitespace();
    let start = self.position;
    let char = self.advance()?;

    let token = match char {
      '\n' => Token::new(TokenKind::Newline, self.span(start)),

      // Delimiters
      '(' => Token::new(TokenKind::LParen, self.span(start)),
      ')' => Token::new(TokenKind::RParen, self.span(start)),
      '{' => Token::new(TokenKind::LBrace, self.span(start)),
      '}' => Token::new(TokenKind::RBrace, self.span(start)),
      '[' => Token::new(TokenKind::LBracket, self.span(start)),
      ']' => Token::new(TokenKind::RBracket, self.span(start)),
      ',' => Token::new(TokenKind::Comma, self.span(start)),
      '@' => Token::new(TokenKind::At, self.span(start)),
      ':' => Token::new(TokenKind::Colon, self.span(start)),
      ';' => Token::new(TokenKind::Semicolon, self.span(start)),

      // Operators
      '+' => {
        let kind = if self.advance_if('+') {
          TokenKind::PlusPlus
        } else if self.advance_if('=') {
          TokenKind::PlusEqual
        } else {
          TokenKind::Plus
        };
        Token::new(kind, self.span(start))
      }
      '-' => {
        let kind = if self.advance_if('-') {
          TokenKind::MinusMinus
        } else if self.advance_if('=') {
          TokenKind::MinusEqual
        } else {
          TokenKind::Minus
        };
        Token::new(kind, self.span(start))
      }
      '*' => {
        let kind = if self.advance_if('*') {
          TokenKind::StarStar
        } else if self.advance_if('=') {
          TokenKind::StarEqual
        } else {
          TokenKind::Star
        };
        Token::new(kind, self.span(start))
      }
      '/' => {
        let kind = if self.advance_if('=') {
          TokenKind::SlashEqual
        } else {
          TokenKind::Slash
        };
        Token::new(kind, self.span(start))
      }
      '%' => {
        let kind = if self.advance_if('=') {
          TokenKind::PercentEqual
        } else {
          TokenKind::Percent
        };
        Token::new(kind, self.span(start))
      }
      '=' => {
        let kind = if self.advance_if('>') {
          TokenKind::Arrow
        } else if self.advance_if('=') {
          TokenKind::EqualEqual
        } else {
          TokenKind::Equal
        };
        Token::new(kind, self.span(start))
      }
      '!' => {
        let kind = if self.advance_if('=') {
          TokenKind::BangEqual
        } else {
          TokenKind::Bang
        };
        Token::new(kind, self.span(start))
      }
      '>' => {
        let kind = if self.advance_if('>') {
          if self.advance_if('=') {
            TokenKind::GreaterGreaterEqual
          } else {
            TokenKind::GreaterGreater
          }
        } else if self.advance_if('=') {
          TokenKind::GreaterThanEqual
        } else {
          TokenKind::GreaterThan
        };
        Token::new(kind, self.span(start))
      }
      '<' => {
        let kind = if self.advance_if('<') {
          if self.advance_if('=') {
            TokenKind::LessLessEqual
          } else {
            TokenKind::LessLess
          }
        } else if self.advance_if('=') {
          TokenKind::LessThanEqual
        } else {
          TokenKind::LessThan
        };
        Token::new(kind, self.span(start))
      }
      '&' => {
        let kind = if self.advance_if('=') {
          TokenKind::AmpersandEqual
        } else if self.advance_if('&') {
          TokenKind::AmpersandAmpersand
        } else {
          TokenKind::Ampersand
        };
        Token::new(kind, self.span(start))
      }
      '|' => {
        let kind = if self.advance_if('=') {
          TokenKind::PipeEqual
        } else if self.advance_if('|') {
          TokenKind::PipePipe
        } else {
          TokenKind::Pipe
        };
        Token::new(kind, self.span(start))
      }
      '^' => {
        let kind = if self.advance_if('=') {
          TokenKind::CaretEqual
        } else {
          TokenKind::Caret
        };
        Token::new(kind, self.span(start))
      }
      '~' => Token::new(TokenKind::Tilde, self.span(start)),
      '.' => {
        let kind = if self.advance_if('.') {
          TokenKind::DotDot
        } else {
          TokenKind::Dot
        };
        Token::new(kind, self.span(start))
      }

      // Strings, Numbers, and Identifiers
      '"' => self.string(start),
      c if c.is_ascii_digit() => self.number(start, c),
      c if c.is_alphabetic() || c == '_' => self.identifier(start, c),

      // Other
      _ => Token::new(TokenKind::Unknown, self.span(start)),
    };

    Some(token)
  }
}
