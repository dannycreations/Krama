use bumpalo::Bump;
use krama_core::ast::literal::Literal;
use krama_core::ast::statement::{Statement, StatementKind};
use krama_core::ast::types::{Type, TypeKind};
use krama_core::span::Span;
use krama_internal::test_parser;

fn expect_const_statement_with_type<'ast>(
  statement: &Statement<'ast>,
  expected_type: Type<'ast>,
) {
  let kind = match &statement.kind {
    StatementKind::Const { kind, .. } => kind,
    _ => panic!("Expected const statement"),
  };
  assert_eq!(kind.as_ref(), Some(&expected_type));
}

test_parser!(
  should_parse_array_type,
  "const a: i32[] = []",
  1,
  |statement: &Statement| {
    let arena = Bump::new();
    let expected_type = Type {
      span: Span::new(9, 14),
      kind: TypeKind::Array {
        element: arena.alloc(Type {
          span: Span::new(9, 12),
          kind: TypeKind::I32,
        }),
        size: None,
      },
    };
    expect_const_statement_with_type(statement, expected_type);
  }
);

test_parser!(
  should_parse_array_type_with_size,
  "const a: i32[5] = []",
  1,
  |statement: &Statement| {
    let arena = Bump::new();
    let expected_type = Type {
      span: Span::new(9, 15),
      kind: TypeKind::Array {
        element: arena.alloc(Type {
          span: Span::new(9, 12),
          kind: TypeKind::I32,
        }),
        size: Some(Literal::Integer(5)),
      },
    };
    expect_const_statement_with_type(statement, expected_type);
  }
);

test_parser!(
  should_parse_tuple_type,
  "const a: [i32, bool] = [1, true]",
  1,
  |statement: &Statement| {
    let arena = bumpalo::Bump::new();
    let expected_type = Type {
      span: Span::new(9, 20),
      kind: TypeKind::Tuple(
        bumpalo::vec![in &arena;
            Type {
                span: Span::new(10, 13),
                kind: TypeKind::I32,
            },
            Type {
                span: Span::new(15, 19),
                kind: TypeKind::Bool,
            }
        ]
        .into(),
      ),
    };
    expect_const_statement_with_type(statement, expected_type);
  }
);

test_parser!(
  should_parse_nested_array_type,
  "const a: i32[][] = []",
  1,
  |statement: &Statement| {
    let arena = Bump::new();
    let expected_type = Type {
      span: Span::new(9, 16),
      kind: TypeKind::Array {
        element: arena.alloc(Type {
          span: Span::new(9, 14),
          kind: TypeKind::Array {
            element: arena.alloc(Type {
              span: Span::new(9, 12),
              kind: TypeKind::I32,
            }),
            size: None,
          },
        }),
        size: None,
      },
    };
    expect_const_statement_with_type(statement, expected_type);
  }
);

test_parser!(
  should_parse_nested_tuple_type,
  "const a: [i32, [bool, str]] = [1, [true, \"a\"]]",
  1,
  |statement: &Statement| {
    let arena = bumpalo::Bump::new();
    let expected_type = Type {
      span: Span::new(9, 27),
      kind: TypeKind::Tuple(
        bumpalo::vec![in &arena;
        Type {
            span: Span::new(10, 13),
            kind: TypeKind::I32,
        },
        Type {
            span: Span::new(15, 26),
            kind: TypeKind::Tuple(
            bumpalo::vec![in &arena;
                Type {
                span: Span::new(16, 20),
                kind: TypeKind::Bool,
                },
                Type {
                span: Span::new(22, 25),
                kind: TypeKind::Str,
                }
            ]
            .into(),
            ),
        }
        ]
        .into(),
      ),
    };
    expect_const_statement_with_type(statement, expected_type);
  }
);
