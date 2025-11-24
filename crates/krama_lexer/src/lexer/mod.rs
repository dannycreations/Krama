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
  pub(super) input_str: &'a str,
  pub(super) input: Peekable<Chars<'a>>,
  pub(super) position: usize,
}

impl<'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Self {
    Self {
      input_str: input,
      input: input.chars().peekable(),
      position: 0,
    }
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
    while let Some(c) = self.peek() {
      if c.is_whitespace() && c != '\n' {
        self.advance();
      } else if c == '/' {
        let mut ahead = self.input.clone();
        ahead.next();
        if ahead.peek() == Some(&'/') {
          self.advance();
          self.advance();
          while let Some(pc) = self.peek() {
            if pc == '\n' {
              break;
            }
            self.advance();
          }
        } else {
          break;
        }
      } else {
        break;
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
        if self.advance_if('+') {
          Token::new(TokenKind::PlusPlus, self.span(start))
        } else if self.advance_if('=') {
          Token::new(TokenKind::PlusEqual, self.span(start))
        } else {
          Token::new(TokenKind::Plus, self.span(start))
        }
      }
      '-' => {
        if self.advance_if('-') {
          Token::new(TokenKind::MinusMinus, self.span(start))
        } else if self.advance_if('=') {
          Token::new(TokenKind::MinusEqual, self.span(start))
        } else {
          Token::new(TokenKind::Minus, self.span(start))
        }
      }
      '*' => {
        if self.advance_if('=') {
          Token::new(TokenKind::StarEqual, self.span(start))
        } else {
          Token::new(TokenKind::Star, self.span(start))
        }
      }
      '/' => {
        if self.advance_if('=') {
          Token::new(TokenKind::SlashEqual, self.span(start))
        } else {
          Token::new(TokenKind::Slash, self.span(start))
        }
      }
      '%' => {
        if self.advance_if('=') {
          Token::new(TokenKind::PercentEqual, self.span(start))
        } else {
          Token::new(TokenKind::Percent, self.span(start))
        }
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
        if self.advance_if('>') {
          if self.advance_if('=') {
            Token::new(TokenKind::GreaterGreaterEqual, self.span(start))
          } else {
            Token::new(TokenKind::GreaterGreater, self.span(start))
          }
        } else if self.advance_if('=') {
          Token::new(TokenKind::GreaterThanEqual, self.span(start))
        } else {
          Token::new(TokenKind::GreaterThan, self.span(start))
        }
      }
      '<' => {
        if self.advance_if('<') {
          if self.advance_if('=') {
            Token::new(TokenKind::LessLessEqual, self.span(start))
          } else {
            Token::new(TokenKind::LessLess, self.span(start))
          }
        } else if self.advance_if('=') {
          Token::new(TokenKind::LessThanEqual, self.span(start))
        } else {
          Token::new(TokenKind::LessThan, self.span(start))
        }
      }
      '&' => {
        if self.advance_if('=') {
          Token::new(TokenKind::AmpersandEqual, self.span(start))
        } else if self.advance_if('&') {
          Token::new(TokenKind::AmpersandAmpersand, self.span(start))
        } else {
          Token::new(TokenKind::Ampersand, self.span(start))
        }
      }
      '|' => {
        if self.advance_if('=') {
          Token::new(TokenKind::PipeEqual, self.span(start))
        } else if self.advance_if('|') {
          Token::new(TokenKind::PipePipe, self.span(start))
        } else {
          Token::new(TokenKind::Pipe, self.span(start))
        }
      }
      '^' => {
        if self.advance_if('=') {
          Token::new(TokenKind::CaretEqual, self.span(start))
        } else {
          Token::new(TokenKind::Caret, self.span(start))
        }
      }
      '~' => Token::new(TokenKind::Tilde, self.span(start)),
      '.' => {
        if self.advance_if('.') {
          Token::new(TokenKind::DotDot, self.span(start))
        } else {
          Token::new(TokenKind::Dot, self.span(start))
        }
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
