mod identifier;
mod number;
mod string;

use std::{iter::Peekable, str::Chars};

use krama_core::{
  span::Span,
  token::{Token, TokenKind},
};

macro_rules! token {
  ($lexer:ident, $start:expr, $kind:expr) => {
    Token::new($kind, $lexer.span($start))
  };

  ($lexer:ident, $start:expr, $one_char:expr, $next_char:expr, $two_chars:expr) => {{
    let kind = if $lexer.advance_if($next_char) {
      $two_chars
    } else {
      $one_char
    };
    token!($lexer, $start, kind)
  }};
}

macro_rules! token_triple {
  ($lexer:ident, $start:expr, $char1:expr, $kind1:expr, $char2:expr, $kind2:expr, $default_kind:expr) => {{
    let kind = if $lexer.advance_if($char1) {
      $kind1
    } else if $lexer.advance_if($char2) {
      $kind2
    } else {
      $default_kind
    };
    token!($lexer, $start, kind)
  }};
}

#[derive(Clone)]
pub struct Lexer<'a> {
  pub(super) input: Peekable<Chars<'a>>,
  pub(super) position: usize,
  pub(super) input_str: &'a str,
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
}

impl<'a> Iterator for Lexer<'a> {
  type Item = Token<'a>;

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      // Skip whitespace
      while let Some(c) = self.peek() {
        if c.is_whitespace() && c != '\n' {
          self.advance();
        } else {
          break;
        }
      }

      // Skip comments
      if self.peek() == Some('/') {
        let mut iter = self.input.clone();
        iter.next();
        if iter.peek() == Some(&'/') {
          // Consume the comment line
          self.advance(); // consume '/'
          self.advance(); // consume '/'
          while let Some(c) = self.peek() {
            if c == '\n' {
              break;
            }
            self.advance();
          }
          continue; // Restart loop to handle more whitespace/comments
        }
      }
      break; // Proceed to tokenization
    }

    let start = self.position;
    let char = self.advance()?;

    let token = match char {
      '\n' => token!(self, start, TokenKind::Newline),

      // Delimiters
      '(' => token!(self, start, TokenKind::LParen),
      ')' => token!(self, start, TokenKind::RParen),
      '{' => token!(self, start, TokenKind::LBrace),
      '}' => token!(self, start, TokenKind::RBrace),
      '[' => token!(self, start, TokenKind::LBracket),
      ']' => token!(self, start, TokenKind::RBracket),
      ',' => token!(self, start, TokenKind::Comma),
      '@' => token!(self, start, TokenKind::At),
      ':' => token!(self, start, TokenKind::Colon),
      ';' => token!(self, start, TokenKind::Semicolon),

      // Operators
      '+' => token_triple!(
        self,
        start,
        '+',
        TokenKind::PlusPlus,
        '=',
        TokenKind::PlusEqual,
        TokenKind::Plus
      ),
      '-' => token_triple!(
        self,
        start,
        '-',
        TokenKind::MinusMinus,
        '=',
        TokenKind::MinusEqual,
        TokenKind::Minus
      ),
      '%' => token!(
        self,
        start,
        TokenKind::Percent,
        '=',
        TokenKind::PercentEqual
      ),
      '/' => token!(self, start, TokenKind::Slash, '=', TokenKind::SlashEqual),
      '*' => token_triple!(
        self,
        start,
        '*',
        TokenKind::StarStar,
        '=',
        TokenKind::StarEqual,
        TokenKind::Star
      ),
      '=' => {
        let kind = if self.advance_if('>') {
          TokenKind::Arrow
        } else if self.advance_if('=') {
          TokenKind::EqualEqual
        } else {
          TokenKind::Equal
        };
        token!(self, start, kind)
      }
      '!' => token!(self, start, TokenKind::Bang, '=', TokenKind::BangEqual),
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
        token!(self, start, kind)
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
        token!(self, start, kind)
      }
      '&' => token_triple!(
        self,
        start,
        '&',
        TokenKind::AmpersandAmpersand,
        '=',
        TokenKind::AmpersandEqual,
        TokenKind::Ampersand
      ),
      '|' => token_triple!(
        self,
        start,
        '|',
        TokenKind::PipePipe,
        '=',
        TokenKind::PipeEqual,
        TokenKind::Pipe
      ),
      '^' => token!(self, start, TokenKind::Caret, '=', TokenKind::CaretEqual),
      '~' => token!(self, start, TokenKind::Tilde),
      '.' => token!(self, start, TokenKind::Dot, '.', TokenKind::DotDot),

      // Strings, Numbers, and Identifiers
      '"' => self.string(start),
      c if c.is_ascii_digit() => self.number(start),
      c if c.is_alphabetic() || c == '_' => self.identifier(start),

      // Other
      _ => token!(self, start, TokenKind::Unknown),
    };

    Some(token)
  }
}
