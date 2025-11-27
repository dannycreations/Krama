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
  pub(super) input: &'a [u8],
  pub(super) position: usize,
  pub(super) input_str: &'a str,
}

impl<'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Self {
    Self {
      input: input.as_bytes(),
      position: 0,
      input_str: input,
    }
  }

  pub fn input_len(&self) -> usize {
    self.input.len()
  }

  fn current_char_and_width(&self) -> Option<(char, usize)> {
    if self.position >= self.input.len() {
      return None;
    }
    // This is safe because we are reading from the original string slice.
    let s =
      unsafe { std::str::from_utf8_unchecked(&self.input[self.position..]) };
    s.chars().next().map(|c| (c, c.len_utf8()))
  }

  pub(super) fn peek(&self) -> Option<char> {
    self.current_char_and_width().map(|(c, _)| c)
  }

  pub(super) fn advance(&mut self) -> Option<char> {
    self.current_char_and_width().map(|(c, width)| {
      self.position += width;
      c
    })
  }

  pub(super) fn advance_if(&mut self, expected: char) -> bool {
    if let Some((c, width)) = self.current_char_and_width() {
      if c == expected {
        self.position += width;
        return true;
      }
    }
    false
  }

  pub(super) fn span(&self, start: usize) -> Span {
    Span::new(start, self.position)
  }

  fn skip_trivia(&mut self) {
    while let Some(c) = self.peek() {
      if c.is_whitespace() && c != '\n' {
        self.advance();
      } else if c == '/' {
        let next_pos = self.position + c.len_utf8();
        if next_pos < self.input.len() && self.input[next_pos] == b'/' {
          self.advance(); // consume '/'
          self.advance(); // consume '/'
          while let Some(c) = self.peek() {
            if c == '\n' {
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
