use bumpalo::collections::Vec as BumpVec;

mod ast;
mod diagnostic;
mod token;
mod value;

pub use ast::*;
pub use diagnostic::*;
pub use token::*;
pub use value::*;

#[derive(Debug, PartialEq)]
pub struct Program<'ast> {
  pub statements: BumpVec<'ast, Statement<'ast>>,
}
