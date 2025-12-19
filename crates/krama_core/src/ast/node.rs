use crate::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Node<'ast, T> {
  pub kind: T,
  pub span: Span<'ast>,
}

impl<'ast, T> Node<'ast, T> {
  pub fn new(kind: T, span: Span<'ast>) -> Self {
    Self { kind, span }
  }
}
