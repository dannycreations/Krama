use bumpalo::Bump;
use krama_lexer::lexer::Lexer;
use krama_parser::parser::Parser;
use krama_runtime::test_parser;

test_parser!(parse_unary_expression, "!true", 1);

test_parser!(parse_binary_expression, "1 + 2", 1);

test_parser!(parse_call_expression, "add(1, 2)", 1);

test_parser!(parse_match_expression, "match (x) { 1 => 2 }", 1);

test_parser!(parse_import_expression, r#"import("std:fs")"#, 1);

test_parser!(
  parse_destructuring_import_expression,
  r#"const { a, b } = import("std:fs")"#,
  1
);

#[test]
fn parse_empty_paren_error() {
  let source = "()";
  let arena = Bump::new();
  let lexer = Lexer::new(source, None);
  let mut parser = Parser::new(lexer, &arena);
  assert!(parser.parse().is_err());
}

#[test]
fn parse_invalid_grouped_expression_error() {
  let source = "(1, 2)";
  let arena = Bump::new();
  let lexer = Lexer::new(source, None);
  let mut parser = Parser::new(lexer, &arena);
  assert!(parser.parse().is_err());
}

#[test]
fn parse_arrow_function_with_no_parameters() {
  let source = "() => 1";
  let arena = Bump::new();
  let lexer = Lexer::new(source, None);
  let mut parser = Parser::new(lexer, &arena);
  let program = parser.parse().unwrap();
  assert_eq!(program.statements.len(), 1);
}

#[test]
fn parse_block_function_with_no_parameters() {
  let source = "() => {}";
  let arena = Bump::new();
  let lexer = Lexer::new(source, None);
  let mut parser = Parser::new(lexer, &arena);
  let program = parser.parse().unwrap();
  assert_eq!(program.statements.len(), 1);
}

#[test]
fn parse_grouped_expression() {
  let source = "(5 + 5)";
  let arena = Bump::new();
  let lexer = Lexer::new(source, None);
  let mut parser = Parser::new(lexer, &arena);
  let program = parser.parse().unwrap();
  assert_eq!(program.statements.len(), 1);
}

#[test]
fn parse_complex_grouped_expression() {
  let source = "(5 + (10 - 2))";
  let arena = Bump::new();
  let lexer = Lexer::new(source, None);
  let mut parser = Parser::new(lexer, &arena);
  let program = parser.parse().unwrap();
  assert_eq!(program.statements.len(), 1);
}
