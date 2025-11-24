use krama_core::ast::statement::{Binding, Statement, StatementKind};
use krama_internal::test_parser;

test_parser!(
  should_parse_let_stmts,
  "let x = 5",
  1,
  |statement: &Statement| {
    match &statement.kind {
      StatementKind::Let { name, .. } => {
        assert_eq!(*name, "x");
      }
      _ => panic!("Expected let statement"),
    }
  }
);

test_parser!(
  should_parse_const_stmts,
  "const x = 5",
  1,
  |statement: &Statement| {
    match &statement.kind {
      StatementKind::Const {
        binding: Binding::Identifier(name),
        ..
      } => {
        assert_eq!(*name, "x");
      }
      _ => panic!("Expected const statement"),
    }
  }
);

test_parser!(
  should_parse_pub_const_stmt,
  "pub const x = 5",
  1,
  |statement: &Statement| {
    match &statement.kind {
      StatementKind::Const {
        public,
        binding: Binding::Identifier(name),
        ..
      } => {
        assert_eq!(*name, "x");
        assert_eq!(*public, true);
      }
      _ => panic!("Expected pub const statement"),
    }
  }
);

test_parser!(
  should_parse_fn_stmt,
  "fn add(a, b) { a + b }",
  1,
  |statement: &Statement| {
    match &statement.kind {
      StatementKind::Fn { name, .. } => {
        assert_eq!(*name, "add");
      }
      _ => panic!("Expected fn statement"),
    }
  }
);

test_parser!(
  should_parse_pub_fn_stmt,
  "pub fn add(a, b) { a + b }",
  1,
  |statement: &Statement| {
    match &statement.kind {
      StatementKind::Fn { public, name, .. } => {
        assert_eq!(*name, "add");
        assert_eq!(*public, true);
      }
      _ => panic!("Expected pub fn statement"),
    }
  }
);

test_parser!(
  should_parse_return_stmt,
  "return 5",
  1,
  |statement: &Statement| {
    match &statement.kind {
      StatementKind::Return { .. } => {}
      _ => panic!("Expected return statement"),
    }
  }
);

test_parser!(
  should_parse_break_stmt,
  "break",
  1,
  |statement: &Statement| {
    match &statement.kind {
      StatementKind::Break => {}
      _ => panic!("Expected break statement"),
    }
  }
);

test_parser!(
  should_parse_continue_stmt,
  "continue",
  1,
  |statement: &Statement| {
    match &statement.kind {
      StatementKind::Continue => {}
      _ => panic!("Expected continue statement"),
    }
  }
);

test_parser!(
  should_parse_while_stmt,
  "while (true) { }",
  1,
  |statement: &Statement| {
    match &statement.kind {
      StatementKind::While { .. } => {}
      _ => panic!("Expected while statement"),
    }
  }
);
