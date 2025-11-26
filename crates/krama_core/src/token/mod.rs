pub mod kind;

pub use kind::TokenKind;

use super::span::Span;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token<'a> {
  pub kind: TokenKind<'a>,
  pub span: Span,
}

impl<'a> Token<'a> {
  pub fn new(kind: TokenKind<'a>, span: Span) -> Self {
    Self { kind, span }
  }
}
