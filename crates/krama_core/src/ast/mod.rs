pub mod expression;
pub mod literal;
pub mod operator;
pub mod statement;
pub mod types;

use crate::ast::statement::Statement;
use bumpalo::collections::Vec as BumpVec;

#[derive(Debug, PartialEq)]
pub struct Program<'ast> {
  pub statements: BumpVec<'ast, Statement<'ast>>,
}
