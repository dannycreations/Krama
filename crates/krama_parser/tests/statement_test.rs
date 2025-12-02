use krama_core::ast::{
  expression::ExpressionKind,
  literal::Literal,
  statement::{Binding, Statement, StatementKind},
};
use krama_runtime::test_parser;

test_parser!(
  parse_let_statement,
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
  parse_const_statement,
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
  parse_public_const_statement,
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
  parse_function_statement,
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
  parse_public_function_statement,
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
  parse_return_statement,
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
  parse_break_statement,
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
  parse_continue_statement,
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
  parse_while_statement,
  "while (true) { }",
  1,
  |statement: &Statement| {
    match &statement.kind {
      StatementKind::While { .. } => {}
      _ => panic!("Expected while statement"),
    }
  }
);

test_parser! {
    simple_object,
    "const foo = { name: \"admin\", age: 20, \"user-id\": 123 }",
    1,
    |statement: &Statement| {
        let expression = match &statement.kind {
            StatementKind::Const { value, .. } => value,
            _ => panic!("Expected const statement"),
        };

        match &expression.kind {
            ExpressionKind::Object { properties } => {
                assert_eq!(properties.len(), 3);

                // Check name property
                let (name_key, name_value) = &properties[0];
                match &name_key.kind {
                    ExpressionKind::Literal(Literal::String(s)) => assert_eq!(*s, "name"),
                    _ => panic!("Expected string literal for key"),
                }
                match &name_value.kind {
                    ExpressionKind::Literal(Literal::String(s)) => assert_eq!(*s, "admin"),
                    _ => panic!("Expected string literal for value"),
                }

                // Check age property
                let (age_key, age_value) = &properties[1];
                match &age_key.kind {
                    ExpressionKind::Literal(Literal::String(s)) => assert_eq!(*s, "age"),
                    _ => panic!("Expected string literal for key"),
                }
                match &age_value.kind {
                    ExpressionKind::Literal(Literal::Integer(i)) => assert_eq!(*i, 20),
                    _ => panic!("Expected integer literal for value"),
                }

                // Check user-id property
                let (user_id_key, user_id_value) = &properties[2];
                match &user_id_key.kind {
                    ExpressionKind::Literal(Literal::String(s)) => assert_eq!(*s, "user-id"),
                    _ => panic!("Expected string literal for key"),
                }
                match &user_id_value.kind {
                    ExpressionKind::Literal(Literal::Integer(i)) => assert_eq!(*i, 123),
                    _ => panic!("Expected integer literal for value"),
                }
            }
            _ => panic!("Expected object literal"),
        }
    }
}
