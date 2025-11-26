use bumpalo::Bump;
use krama_internal::test_parser;
use krama_lexer::lexer::Lexer;
use krama_parser::parser::Parser;

test_parser!(should_parse_unary_exprs, "!true", 1);

test_parser!(should_parse_binary_exprs, "1 + 2", 1);

test_parser!(should_parse_call_expr, "add(1, 2)", 1);

test_parser!(should_parse_match_expr, "match (x) { 1 => 2 }", 1);

test_parser!(should_parse_import_expr, r#"@import("std:fs")"#, 1);

test_parser!(
  should_parse_destructuring_import_expr,
  r#"const { a, b } = @import("std:fs")"#,
  1
);

#[test]
fn should_empty_paren_error() {
  let source = "()";
  let arena = Bump::new();
  let lexer = Lexer::new(source);
  let mut parser = Parser::new(lexer, &arena);
  assert!(parser.parse().is_err());
}

#[test]
fn should_invalid_grouped_expression_error() {
  let source = "(1, 2)";
  let arena = Bump::new();
  let lexer = Lexer::new(source);
  let mut parser = Parser::new(lexer, &arena);
  assert!(parser.parse().is_err());
}

#[test]
fn should_parse_arrow_function_no_params() {
  let source = "() => 1";
  let arena = Bump::new();
  let lexer = Lexer::new(source);
  let mut parser = Parser::new(lexer, &arena);
  let program = parser.parse().unwrap();
  assert_eq!(program.statements.len(), 1);
}

#[test]
fn should_parse_block_function_no_params() {
  let source = "() => {}";
  let arena = Bump::new();
  let lexer = Lexer::new(source);
  let mut parser = Parser::new(lexer, &arena);
  let program = parser.parse().unwrap();
  assert_eq!(program.statements.len(), 1);
}

#[test]
fn should_parse_grouped_expression() {
  let source = "(5 + 5)";
  let arena = Bump::new();
  let lexer = Lexer::new(source);
  let mut parser = Parser::new(lexer, &arena);
  let program = parser.parse().unwrap();
  assert_eq!(program.statements.len(), 1);
}

#[test]
fn should_parse_complex_grouped_expression() {
  let source = "(5 + (10 - 2))";
  let arena = Bump::new();
  let lexer = Lexer::new(source);
  let mut parser = Parser::new(lexer, &arena);
  let program = parser.parse().unwrap();
  assert_eq!(program.statements.len(), 1);
}
