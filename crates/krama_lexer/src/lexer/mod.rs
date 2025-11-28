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
    unsafe { std::str::from_utf8_unchecked(&self.input[start..end]) }
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
        Some(c) if c.is_whitespace() && c != '\n' => {
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
}

impl<'a> Iterator for Lexer<'a> {
  type Item = Token<'a>;

  fn next(&mut self) -> Option<Self::Item> {
    self.skip_trivia();

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
      '%' => {
        token!(
          self,
          start,
          TokenKind::Percent,
          '=',
          TokenKind::PercentEqual
        )
      }
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
