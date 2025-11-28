use krama_core::ast::{
  expression::ExpressionKind,
  literal::Literal,
  statement::{Statement, StatementKind},
};
use krama_internal::test_parser;

test_parser!(
  parse_empty_collection_expression,
  "[]",
  1,
  |statement: &Statement| {
    let expression = match &statement.kind {
      StatementKind::Expression { expression } => expression,
      _ => panic!("Expected expression statement"),
    };
    match &expression.kind {
      ExpressionKind::Collection { elements } => {
        assert_eq!(elements.len(), 0);
      }
      _ => panic!("Expected array literal"),
    }
  }
);

test_parser!(
  parse_collection_expression_with_one_element,
  "[1]",
  1,
  |statement: &Statement| {
    let expression = match &statement.kind {
      StatementKind::Expression { expression } => expression,
      _ => panic!("Expected expression statement"),
    };
    match &expression.kind {
      ExpressionKind::Collection { elements } => {
        assert_eq!(elements.len(), 1);
        match &elements[0].kind {
          ExpressionKind::Literal(literal) => {
            assert_eq!(literal, &Literal::Integer(1))
          }
          _ => panic!("Expected literal"),
        }
      }
      _ => panic!("Expected tuple literal"),
    }
  }
);

test_parser!(
  parse_collection_expression_with_multiple_elements,
  "[1, true, \"hello\"]",
  1,
  |statement: &Statement| {
    let expression = match &statement.kind {
      StatementKind::Expression { expression } => expression,
      _ => panic!("Expected expression statement"),
    };
    match &expression.kind {
      ExpressionKind::Collection { elements } => {
        assert_eq!(elements.len(), 3);
        match &elements[0].kind {
          ExpressionKind::Literal(literal) => {
            assert_eq!(literal, &Literal::Integer(1))
          }
          _ => panic!("Expected literal"),
        }
        match &elements[1].kind {
          ExpressionKind::Literal(literal) => {
            assert_eq!(literal, &Literal::Boolean(true))
          }
          _ => panic!("Expected literal"),
        }
        match &elements[2].kind {
          ExpressionKind::Literal(literal) => {
            assert_eq!(literal, &Literal::String("hello"))
          }
          _ => panic!("Expected literal"),
        }
      }
      _ => panic!("Expected tuple literal"),
    }
  }
);
