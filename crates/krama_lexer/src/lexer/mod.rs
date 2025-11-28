mod identifier;
mod number;
mod string;

use krama_core::{
  span::Span,
  token::{Token, TokenKind},
};
use phf::{phf_map, Map};

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

static DELIMITERS: Map<char, TokenKind> = phf_map! {
    '(' => TokenKind::LParen,
    ')' => TokenKind::RParen,
    '{' => TokenKind::LBrace,
    '}' => TokenKind::RBrace,
    '[' => TokenKind::LBracket,
    ']' => TokenKind::RBracket,
    ',' => TokenKind::Comma,
    '@' => TokenKind::At,
    ':' => TokenKind::Colon,
    ';' => TokenKind::Semicolon,
    '~' => TokenKind::Tilde,
};

static OPERATORS: [char; 13] = [
  '%', '/', '!', '^', '.', '+', '-', '*', '&', '|', '=', '>', '<',
];

#[derive(Clone)]
pub struct Lexer<'a> {
  input: &'a [u8],
  position: usize,
}

impl<'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Self {
    Self {
      input: input.as_bytes(),
      position: 0,
    }
  }

  pub fn input_len(&self) -> usize {
    self.input.len()
  }

  pub(super) fn peek(&mut self) -> Option<char> {
    if self.position >= self.input.len() {
      return None;
    }
    Some(self.input[self.position] as char)
  }

  pub(super) fn peek_next(&self) -> Option<char> {
    if self.position + 1 >= self.input.len() {
      return None;
    }
    Some(self.input[self.position + 1] as char)
  }

  pub(super) fn advance(&mut self) -> Option<char> {
    if self.position >= self.input.len() {
      return None;
    }
    let ch = self.input[self.position] as char;
    self.position += 1;
    Some(ch)
  }

  pub(super) fn advance_if(&mut self, expected: char) -> bool {
    if self.position >= self.input.len() {
      return false;
    }
    if self.input[self.position] as char == expected {
      self.position += 1;
      true
    } else {
      false
    }
  }

  pub(super) fn span(&self, start: usize) -> Span {
    Span::new(start, self.position)
  }

  pub(super) fn slice(&self, start: usize, end: usize) -> &'a str {
    std::str::from_utf8(&self.input[start..end]).unwrap()
  }

  fn skip_line_comment(&mut self) {
    self.advance();
    self.advance();
    while self.peek().is_some_and(|c| c != '\n') {
      self.advance();
    }
  }

  fn skip_block_comment(&mut self) {
    self.advance();
    self.advance();
    while let Some(c) = self.advance() {
      if c == '*' && self.peek() == Some('/') {
        self.advance();
        break;
      }
    }
  }

  fn skip_trivia(&mut self) {
    loop {
      match self.peek() {
        Some(c) if c.is_whitespace() => {
          self.advance();
        }
        Some('/') => match self.peek_next() {
          Some('/') => self.skip_line_comment(),
          Some('*') => self.skip_block_comment(),
          _ => break,
        },
        _ => break,
      }
    }
  }

  fn operator(&mut self, start: usize, char: char) -> Token<'a> {
    match char {
      '%' => token!(
        self,
        start,
        TokenKind::Percent,
        '=',
        TokenKind::PercentEqual
      ),
      '/' => token!(self, start, TokenKind::Slash, '=', TokenKind::SlashEqual),
      '!' => token!(self, start, TokenKind::Bang, '=', TokenKind::BangEqual),
      '^' => token!(self, start, TokenKind::Caret, '=', TokenKind::CaretEqual),
      '.' => token!(self, start, TokenKind::Dot, '.', TokenKind::DotDot),
      '+' => {
        let kind = if self.advance_if('+') {
          TokenKind::PlusPlus
        } else if self.advance_if('=') {
          TokenKind::PlusEqual
        } else {
          TokenKind::Plus
        };
        token!(self, start, kind)
      }
      '-' => {
        let kind = if self.advance_if('-') {
          TokenKind::MinusMinus
        } else if self.advance_if('=') {
          TokenKind::MinusEqual
        } else {
          TokenKind::Minus
        };
        token!(self, start, kind)
      }
      '*' => {
        let kind = if self.advance_if('*') {
          TokenKind::StarStar
        } else if self.advance_if('=') {
          TokenKind::StarEqual
        } else {
          TokenKind::Star
        };
        token!(self, start, kind)
      }
      '&' => {
        let kind = if self.advance_if('&') {
          TokenKind::AmpersandAmpersand
        } else if self.advance_if('=') {
          TokenKind::AmpersandEqual
        } else {
          TokenKind::Ampersand
        };
        token!(self, start, kind)
      }
      '|' => {
        let kind = if self.advance_if('|') {
          TokenKind::PipePipe
        } else if self.advance_if('=') {
          TokenKind::PipeEqual
        } else {
          TokenKind::Pipe
        };
        token!(self, start, kind)
      }
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
      _ => unreachable!(),
    }
  }
}

impl<'a> Iterator for Lexer<'a> {
  type Item = Token<'a>;

  fn next(&mut self) -> Option<Self::Item> {
    self.skip_trivia();

    let start = self.position;
    let char = self.advance()?;

    let token = if let Some(kind) = DELIMITERS.get(&char) {
      token!(self, start, *kind)
    } else {
      match char {
        c if OPERATORS.contains(&c) => self.operator(start, char),
        '"' => self.string(start),
        c if c.is_ascii_digit() => self.number(start),
        c if c.is_alphabetic() || c == '_' => self.identifier(start),
        _ => token!(self, start, TokenKind::Unknown),
      }
    };

    Some(token)
  }
}
