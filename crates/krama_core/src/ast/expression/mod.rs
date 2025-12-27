use crate::Node;

mod kind;
pub use kind::*;

pub type Expression = Node<ExpressionKind>;
