use bumpalo::Bump;
use krama_core::{ExpressionKind, Literal, Span, Statement, StatementKind};
use krama_runtime::{test_parser, Lexer, Parser};

test_parser!(
  parse_statement_with_semicolon,
  "1;",
  1,
  |statement: &Statement| {
    assert_eq!(statement.span, Span::new(0, 1, Some("1;"), None));
    match &statement.kind {
      StatementKind::Expression { expression } => {
        assert_eq!(expression.span, Span::new(0, 1, Some("1;"), None));
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
  parse_statement_without_semicolon,
  "1",
  1,
  |statement: &Statement| {
    assert_eq!(statement.span, Span::new(0, 1, Some("1"), None));
    match &statement.kind {
      StatementKind::Expression { expression } => {
        assert_eq!(expression.span, Span::new(0, 1, Some("1"), None));
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
fn parse_multiple_statements_with_and_without_semicolon() {
  let text = "1;2\n3";
  let arena = Bump::new();
  let lexer = Lexer::new(text, None);
  let mut parser = Parser::new(lexer, &arena);
  let program = parser.parse().unwrap();
  assert_eq!(program.statements.len(), 3);

  let stmt1 = &program.statements[0];
  assert_eq!(stmt1.span, Span::new(0, 1, Some(text), None));
  match &stmt1.kind {
    StatementKind::Expression { expression } => {
      assert_eq!(expression.span, Span::new(0, 1, Some(text), None));
      assert!(matches!(
        &expression.kind,
        ExpressionKind::Literal(Literal::Integer(1))
      ));
    }
    _ => panic!("Expected expression statement"),
  }

  let stmt2 = &program.statements[1];
  assert_eq!(stmt2.span, Span::new(2, 3, Some(text), None));
  match &stmt2.kind {
    StatementKind::Expression { expression } => {
      assert_eq!(expression.span, Span::new(2, 3, Some(text), None));
      assert!(matches!(
        &expression.kind,
        ExpressionKind::Literal(Literal::Integer(2))
      ));
    }
    _ => panic!("Expected expression statement"),
  }

  let stmt3 = &program.statements[2];
  assert_eq!(stmt3.span, Span::new(4, 5, Some(text), None));
  match &stmt3.kind {
    StatementKind::Expression { expression } => {
      assert_eq!(expression.span, Span::new(4, 5, Some(text), None));
      assert!(matches!(
        &expression.kind,
        ExpressionKind::Literal(Literal::Integer(3))
      ));
    }
    _ => panic!("Expected expression statement"),
  }
}

#[test]
fn parse_multiple_statements_with_trailing_semicolon() {
  let text = "1;2;\n3;";
  let arena = Bump::new();
  let lexer = Lexer::new(text, None);
  let mut parser = Parser::new(lexer, &arena);
  let program = parser.parse().unwrap();
  assert_eq!(program.statements.len(), 3);

  let stmt1 = &program.statements[0];
  assert_eq!(stmt1.span, Span::new(0, 1, Some(text), None));
  match &stmt1.kind {
    StatementKind::Expression { expression } => {
      assert_eq!(expression.span, Span::new(0, 1, Some(text), None));
      assert!(matches!(
        &expression.kind,
        ExpressionKind::Literal(Literal::Integer(1))
      ));
    }
    _ => panic!("Expected expression statement"),
  }

  let stmt2 = &program.statements[1];
  assert_eq!(stmt2.span, Span::new(2, 3, Some(text), None));
  match &stmt2.kind {
    StatementKind::Expression { expression } => {
      assert_eq!(expression.span, Span::new(2, 3, Some(text), None));
      assert!(matches!(
        &expression.kind,
        ExpressionKind::Literal(Literal::Integer(2))
      ));
    }
    _ => panic!("Expected expression statement"),
  }

  let stmt3 = &program.statements[2];
  assert_eq!(stmt3.span, Span::new(5, 6, Some(text), None));
  match &stmt3.kind {
    StatementKind::Expression { expression } => {
      assert_eq!(expression.span, Span::new(5, 6, Some(text), None));
      assert!(matches!(
        &expression.kind,
        ExpressionKind::Literal(Literal::Integer(3))
      ));
    }
    _ => panic!("Expected expression statement"),
  }
}
