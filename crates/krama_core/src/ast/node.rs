use std::{
  fmt::{Display, Formatter, Result as FmtResult},
  marker::PhantomData,
};

use crate::Span;

/// A generic node in the AST, wrapping a kind with its source span.
/// Optimized with #[repr(C)] to ensure predictable layout and minimal padding.
#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Node<'ast, T> {
  /// The specific data for this node (e.g., ExpressionKind or StatementKind).
  pub kind: T,
  /// The source code range this node covers.
  pub span: Span,
  /// Ensures the lifetime 'ast is bound to the Node.
  pub _marker: PhantomData<&'ast ()>,
}

impl<'ast, T> Node<'ast, T> {
  /// Creates a new AST node.
  #[inline(always)]
  pub fn new(kind: T, span: Span) -> Self {
    Self {
      kind,
      span,
      _marker: PhantomData,
    }
  }

  /// Returns the span of this node.
  #[inline(always)]
  pub fn span(&self) -> Span {
    self.span
  }
}

/// Helper trait for types that can be wrapped in a Node.
pub trait IntoNode<'ast>: Sized {
  fn into_node(self, span: Span) -> Node<'ast, Self> {
    Node::new(self, span)
  }
}

impl<'ast, T> IntoNode<'ast> for T {}

impl<'ast, T> Display for Node<'ast, T>
where
  T: Display,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}", self.kind)
  }
}
