use bumpalo::Bump;
use krama_core::{
  ast::{
    literal::Literal,
    statement::{Statement, StatementKind},
    types::{Type, TypeKind},
  },
  span::Span,
};
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
  parse_array_type,
  "const a: i32[] = []",
  1,
  |statement: &Statement| {
    let arena = Bump::new();
    let expected_type = Type::new(
      TypeKind::Array {
        element: arena.alloc(Type::new(TypeKind::I32, Span::new(9, 12))),
        size: None,
      },
      Span::new(9, 14),
    );
    expect_const_statement_with_type(statement, expected_type);
  }
);

test_parser!(
  parse_array_type_with_size,
  "const a: i32[5] = []",
  1,
  |statement: &Statement| {
    let arena = Bump::new();
    let expected_type = Type::new(
      TypeKind::Array {
        element: arena.alloc(Type::new(TypeKind::I32, Span::new(9, 12))),
        size: Some(Literal::Integer(5)),
      },
      Span::new(9, 15),
    );
    expect_const_statement_with_type(statement, expected_type);
  }
);

test_parser!(
  parse_tuple_type,
  "const a: [i32, bool] = [1, true]",
  1,
  |statement: &Statement| {
    let arena = Bump::new();
    let expected_type = Type::new(
      TypeKind::Tuple(
        bumpalo::vec![in &arena;
            Type::new(TypeKind::I32, Span::new(10, 13)),
            Type::new(TypeKind::Bool, Span::new(15, 19))
        ]
        .into(),
      ),
      Span::new(9, 20),
    );
    expect_const_statement_with_type(statement, expected_type);
  }
);

test_parser!(
  parse_nested_array_type,
  "const a: i32[][] = []",
  1,
  |statement: &Statement| {
    let arena = Bump::new();
    let expected_type = Type::new(
      TypeKind::Array {
        element: arena.alloc(Type::new(
          TypeKind::Array {
            element: arena.alloc(Type::new(TypeKind::I32, Span::new(9, 12))),
            size: None,
          },
          Span::new(9, 14),
        )),
        size: None,
      },
      Span::new(9, 16),
    );
    expect_const_statement_with_type(statement, expected_type);
  }
);

test_parser!(
  parse_nested_tuple_type,
  "const a: [i32, [bool, str]] = [1, [true, \"a\"]]",
  1,
  |statement: &Statement| {
    let arena = Bump::new();
    let expected_type = Type::new(
      TypeKind::Tuple(
        bumpalo::vec![in &arena;
        Type::new(TypeKind::I32, Span::new(10, 13)),
        Type::new(
            TypeKind::Tuple(
            bumpalo::vec![in &arena;
                Type::new(TypeKind::Bool, Span::new(16, 20)),
                Type::new(TypeKind::Str, Span::new(22, 25))
            ]
            .into(),
            ),
            Span::new(15, 26),
        )
        ]
        .into(),
      ),
      Span::new(9, 27),
    );
    expect_const_statement_with_type(statement, expected_type);
  }
);
