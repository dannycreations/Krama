mod display;
mod kind;

pub use kind::*;

use super::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
  pub kind: TokenKind,
  pub span: Span,
}

impl Token {
  pub fn new(kind: TokenKind, span: Span) -> Self {
    Self { kind, span }
  }
}
