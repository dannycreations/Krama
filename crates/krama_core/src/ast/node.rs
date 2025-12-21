use crate::Span;

/// A generic node in the AST, wrapping a kind with its source span.
/// Optimized with #[repr(C)] to ensure predictable layout and minimal padding.
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Node<'ast, T> {
  pub kind: T,
  pub span: Span,
  pub _marker: std::marker::PhantomData<&'ast ()>,
}

impl<'ast, T> Node<'ast, T> {
  /// Creates a new AST node.
  #[inline(always)]
  pub fn new(kind: T, span: Span) -> Self {
    Self {
      kind,
      span,
      _marker: std::marker::PhantomData,
    }
  }
}
