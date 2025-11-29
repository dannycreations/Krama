pub mod kind;
pub use kind::{TokenKind, KEYWORDS};

use super::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
  pub kind: TokenKind<'a>,
  pub span: Span<'a>,
}

impl<'a> Token<'a> {
  pub fn new(kind: TokenKind<'a>, span: Span<'a>) -> Self {
    Self { kind, span }
  }
}
