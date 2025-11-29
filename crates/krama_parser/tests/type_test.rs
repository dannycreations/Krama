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
        element: arena.alloc(Type::new(
          TypeKind::I32,
          Span::new(9, 12, Some("const a: i32[] = []"), None),
        )),
        size: None,
      },
      Span::new(9, 14, Some("const a: i32[] = []"), None),
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
        element: arena.alloc(Type::new(
          TypeKind::I32,
          Span::new(9, 12, Some("const a: i32[5] = []"), None),
        )),
        size: Some(Literal::Integer(5)),
      },
      Span::new(9, 15, Some("const a: i32[5] = []"), None),
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
            Type::new(TypeKind::I32, Span::new(10, 13, Some("const a: [i32, bool] = [1, true]"), None)),
            Type::new(TypeKind::Bool, Span::new(15, 19, Some("const a: [i32, bool] = [1, true]"), None))
        ]
        .into(),
      ),
      Span::new(9, 20, Some("const a: [i32, bool] = [1, true]"), None),
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
            element: arena.alloc(Type::new(
              TypeKind::I32,
              Span::new(9, 12, Some("const a: i32[][] = []"), None),
            )),
            size: None,
          },
          Span::new(9, 14, Some("const a: i32[][] = []"), None),
        )),
        size: None,
      },
      Span::new(9, 16, Some("const a: i32[][] = []"), None),
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
        Type::new(TypeKind::I32, Span::new(10, 13, Some("const a: [i32, [bool, str]] = [1, [true, \"a\"]]"), None)),
        Type::new(
            TypeKind::Tuple(
            bumpalo::vec![in &arena;
                Type::new(TypeKind::Bool, Span::new(16, 20, Some("const a: [i32, [bool, str]] = [1, [true, \"a\"]]"), None)),
                Type::new(TypeKind::Str, Span::new(22, 25, Some("const a: [i32, [bool, str]] = [1, [true, \"a\"]]"), None))
            ]
            .into(),
            ),
            Span::new(15, 26, Some("const a: [i32, [bool, str]] = [1, [true, \"a\"]]"), None),
        )
        ]
        .into(),
      ),
      Span::new(9, 27, Some("const a: [i32, [bool, str]] = [1, [true, \"a\"]]"), None),
    );
    expect_const_statement_with_type(statement, expected_type);
  }
);
