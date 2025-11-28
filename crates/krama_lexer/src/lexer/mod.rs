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

#[derive(Clone)]
pub struct Lexer<'a> {
  input: &'a str,
  position: usize,
}

impl<'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Self {
    Self { input, position: 0 }
  }

  pub fn input_len(&self) -> usize {
    self.input.len()
  }

  fn current_char(&self) -> Option<(char, usize)> {
    if self.position >= self.input.len() {
      return None;
    }
    self.input[self.position..]
      .chars()
      .next()
      .map(|c| (c, c.len_utf8()))
  }

  pub(super) fn peek(&self) -> Option<char> {
    self.current_char().map(|(c, _)| c)
  }

  pub(super) fn peek_next(&self) -> Option<char> {
    let (_, current_len) = self.current_char()?;
    let next_pos = self.position + current_len;
    if next_pos >= self.input.len() {
      return None;
    }
    self.input[next_pos..].chars().next()
  }

  pub(super) fn advance(&mut self) -> Option<char> {
    let (c, len) = self.current_char()?;
    self.position += len;
    Some(c)
  }

  pub(super) fn advance_if(&mut self, expected: char) -> bool {
    if let Some((c, len)) = self.current_char() {
      if c == expected {
        self.position += len;
        return true;
      }
    }
    false
  }

  pub(super) fn span(&self, start: usize) -> Span {
    Span::new(start, self.position)
  }

  pub(super) fn slice(&self, start: usize, end: usize) -> &'a str {
    &self.input[start..end]
  }

  fn skip_trivia(&mut self) {
    loop {
      match self.peek() {
        Some(c) if c.is_whitespace() => {
          self.advance();
        }
        Some('/') => {
          if self.peek_next() == Some('/') {
            while self.peek().is_some_and(|c| c != '\n') {
              self.advance();
            }
          } else if self.peek_next() == Some('*') {
            self.advance();
            self.advance();
            while self.peek().is_some_and(|c| c != '*')
              || self.peek_next().is_some_and(|c| c != '/')
            {
              self.advance();
            }
            self.advance();
            self.advance();
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
    let char = self.advance()?;

    let token = match char {
      '(' => token!(self, start, TokenKind::LParen),
      ')' => token!(self, start, TokenKind::RParen),
      '{' => token!(self, start, TokenKind::LBrace),
      '}' => token!(self, start, TokenKind::RBrace),
      '[' => token!(self, start, TokenKind::LBracket),
      ']' => token!(self, start, TokenKind::RBracket),
      ',' => token!(self, start, TokenKind::Comma),
      ':' => token!(self, start, TokenKind::Colon),
      ';' => token!(self, start, TokenKind::Semicolon),
      '~' => token!(self, start, TokenKind::Tilde),
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
      '"' => self.string(start),
      c if c.is_ascii_digit() => self.number(start),
      c if c.is_alphabetic() || c == '_' => self.identifier(start),
      _ => token!(self, start, TokenKind::Unknown),
    };

    Some(token)
  }
}
