use krama_core::token::{Token, TokenKind};

use super::Lexer;

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

impl<'a> Lexer<'a> {
  pub(super) fn punctuator(&mut self, start: usize, byte: u8) -> Token<'a> {
    match byte {
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
      _ => token!(self, start, TokenKind::Unknown),
    }
  }
}
