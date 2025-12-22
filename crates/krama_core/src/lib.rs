use bumpalo::collections::Vec as BumpVec;

mod ast;
mod error;
mod object;
mod token;

pub use ast::*;
pub use error::*;
pub use object::*;
pub use token::*;

#[derive(Debug, PartialEq)]
pub struct Program<'ast> {
  pub statements: BumpVec<'ast, Statement<'ast>>,
}
