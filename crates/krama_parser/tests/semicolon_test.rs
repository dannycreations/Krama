use krama_core::ast::expression::ExpressionKind;
use krama_core::ast::literal::Literal;
use krama_core::ast::statement::{Statement, StatementKind};
use krama_core::span::Span;
use krama_internal::test_parser;

test_parser!(
  test_semicolon_statement,
  "1;",
  1,
  |statement: &Statement| {
    assert_eq!(statement.span, Span::new(0, 1));
    match &statement.kind {
      StatementKind::Expression { expression } => {
        assert_eq!(expression.span, Span::new(0, 1));
        matches!(
          &expression.kind,
          ExpressionKind::Literal(Literal::Integer(1))
        );
      }
      _ => panic!("Expected expression statement"),
    }
  }
);

test_parser!(
  test_semicolon_statement_without_semicolon,
  "1",
  1,
  |statement: &Statement| {
    assert_eq!(statement.span, Span::new(0, 1));
    match &statement.kind {
      StatementKind::Expression { expression } => {
        assert_eq!(expression.span, Span::new(0, 1));
        matches!(
          &expression.kind,
          ExpressionKind::Literal(Literal::Integer(1))
        );
      }
      _ => panic!("Expected expression statement"),
    }
  }
);

#[test]
fn test_multiple_statements_with_and_without_semicolon() {
  let text = "1;2\n3";
  let arena = bumpalo::Bump::new();
  let lexer = krama_lexer::lexer::Lexer::new(text);
  let mut parser = krama_parser::parser::Parser::new(lexer, &arena);
  let program = parser.parse().unwrap();
  assert_eq!(program.statements.len(), 3);

  let stmt1 = &program.statements[0];
  assert_eq!(stmt1.span, Span::new(0, 1));
  match &stmt1.kind {
    StatementKind::Expression { expression } => {
      assert_eq!(expression.span, Span::new(0, 1));
      assert!(matches!(
        &expression.kind,
        ExpressionKind::Literal(Literal::Integer(1))
      ));
    }
    _ => panic!("Expected expression statement"),
  }

  let stmt2 = &program.statements[1];
  assert_eq!(stmt2.span, Span::new(2, 3));
  match &stmt2.kind {
    StatementKind::Expression { expression } => {
      assert_eq!(expression.span, Span::new(2, 3));
      assert!(matches!(
        &expression.kind,
        ExpressionKind::Literal(Literal::Integer(2))
      ));
    }
    _ => panic!("Expected expression statement"),
  }

  let stmt3 = &program.statements[2];
  assert_eq!(stmt3.span, Span::new(4, 5));
  match &stmt3.kind {
    StatementKind::Expression { expression } => {
      assert_eq!(expression.span, Span::new(4, 5));
      assert!(matches!(
        &expression.kind,
        ExpressionKind::Literal(Literal::Integer(3))
      ));
    }
    _ => panic!("Expected expression statement"),
  }
}
