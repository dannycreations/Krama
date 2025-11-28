pub mod expression;
pub mod literal;
pub mod operator;
pub mod precedence;
pub mod statement;
pub mod types;

use std::marker::PhantomData;

use bumpalo::collections::Vec as BumpVec;

use crate::{ast::statement::Statement, span::Span};

#[derive(Debug, PartialEq)]
pub struct Program<'ast> {
  pub statements: BumpVec<'ast, Statement<'ast>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node<'ast, T> {
  pub kind: T,
  pub span: Span,
  _phantom: PhantomData<&'ast ()>,
}

impl<'ast, T> Node<'ast, T> {
  pub fn new(kind: T, span: Span) -> Self {
    Self {
      kind,
      span,
      _phantom: PhantomData,
    }
  }
}
