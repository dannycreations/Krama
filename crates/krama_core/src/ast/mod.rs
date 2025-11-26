pub mod expression;
pub mod literal;
pub mod operator;
pub mod statement;
pub mod types;

use bumpalo::collections::Vec as BumpVec;

use crate::ast::statement::Statement;

#[derive(Debug, PartialEq)]
pub struct Program<'ast> {
  pub statements: BumpVec<'ast, Statement<'ast>>,
}
