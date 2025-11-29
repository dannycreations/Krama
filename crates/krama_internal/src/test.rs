#[macro_export]
macro_rules! resolve_future {
  ($result:expr) => {
    if let ::krama_core::object::Object::Future(future_cell) = $result {
      let future = future_cell.borrow_mut().take();
      if let Some(future) = future {
        future.await
      } else {
        Err(::krama_core::error::Error {
          span: Default::default(),
          kind: ::krama_core::error::ErrorKind::RuntimeError(
            "Future already resolved".to_string(),
          ),
          file_path: None,
          source: None,
        })
      }
    } else {
      Ok($result)
    }
  };
}

#[macro_export]
macro_rules! test_eval {
  ($name:ident, $source:expr, $expected:expr) => {
    #[tokio::test]
    async fn $name() {
      let arena = ::bumpalo::Bump::new();
      let interpreter =
        ::krama_runtime::interpreter::Interpreter::new(&arena, None);
      let source = arena.alloc_str($source);
      let result = interpreter.eval(source).await.unwrap();
      let result = $crate::resolve_future!(result).unwrap();
      assert_eq!(result, $expected);
    }
  };
}

#[macro_export]
macro_rules! test_eval_is_native_function {
  ($name:ident, $source:expr) => {
    #[tokio::test]
    async fn $name() {
      let arena = ::bumpalo::Bump::new();
      let interpreter =
        ::krama_runtime::interpreter::Interpreter::new(&arena, None);
      let source = arena.alloc_str($source);
      let result = interpreter.eval(source).await.unwrap();
      let result = $crate::resolve_future!(result).unwrap();
      assert!(matches!(
        result,
        ::krama_core::object::Object::Function(
          ::krama_core::object::Function::Native(_)
        )
      ));
    }
  };
}

#[macro_export]
macro_rules! test_eval_error {
  ($name:ident, $source:expr, $expected:pat) => {
    #[tokio::test]
    async fn $name() {
      let arena = ::bumpalo::Bump::new();
      let interpreter =
        ::krama_runtime::interpreter::Interpreter::new(&arena, None);
      let source = arena.alloc_str($source);
      let result = interpreter.eval(source).await;
      assert!(matches!(result.unwrap_err().kind, $expected));
    }
  };

  ($name:ident, $source:expr) => {
    #[tokio::test]
    async fn $name() {
      let arena = ::bumpalo::Bump::new();
      let interpreter =
        ::krama_runtime::interpreter::Interpreter::new(&arena, None);
      let source = arena.alloc_str($source);
      let result = interpreter.eval(source).await;
      assert!(result.is_err());
    }
  };
}

#[macro_export]
macro_rules! test_eval_is_module {
  ($name:ident, $source:expr, $expected:expr) => {
    #[tokio::test]
    async fn $name() {
      let arena = ::bumpalo::Bump::new();
      let interpreter =
        ::krama_runtime::interpreter::Interpreter::new(&arena, None);
      let source = arena.alloc_str($source);
      let result = interpreter.eval(source).await.unwrap();
      let result = $crate::resolve_future!(result).unwrap();
      if let ::krama_core::object::Object::Scope(module) = result {
        assert_eq!(module.name, $expected);
      } else {
        panic!("Expected a module object, but got {:?}", result);
      }
    }
  };
}

#[macro_export]
macro_rules! test_eval_with_file {
  ($name:ident, $filename:expr, $file_content:expr, $source:expr, $expected:expr) => {
    #[tokio::test]
    async fn $name() {
      let arena = ::bumpalo::Bump::new();
      let interpreter =
        ::krama_runtime::interpreter::Interpreter::new(&arena, Some(""));
      let input = arena.alloc_str($source);

      ::tokio::fs::write($filename, $file_content).await.unwrap();
      let evaluated = interpreter.eval(input).await.unwrap();
      let evaluated = $crate::resolve_future!(evaluated).unwrap();
      ::tokio::fs::remove_file($filename).await.unwrap();

      assert_eq!(evaluated, $expected);
    }
  };
}

#[macro_export]
macro_rules! test_lexer {
  ($name:ident, $input:expr, $expected:expr) => {
    #[test]
    fn $name() {
      let lexer = ::krama_lexer::lexer::Lexer::new($input);
      let tokens: Vec<::krama_core::token::Token> = lexer.collect();
      let kinds: Vec<::krama_core::token::TokenKind> =
        tokens.into_iter().map(|t| t.kind).collect();
      assert_eq!(kinds, $expected);
    }
  };
}

#[macro_export]
macro_rules! test_lexer_single {
  ($name:ident, $input:expr, $expected:expr) => {
    #[test]
    fn $name() {
      let mut lexer = ::krama_lexer::lexer::Lexer::new($input);
      let token = lexer.next().unwrap();
      assert_eq!(token.kind, $expected);
    }
  };
}

#[macro_export]
macro_rules! test_parser {
  ($name:ident, $source:expr, $len:expr) => {
    #[test]
    fn $name() {
      let arena = ::bumpalo::Bump::new();
      let lexer = ::krama_lexer::lexer::Lexer::new($source);
      let mut parser = ::krama_parser::parser::Parser::new(lexer, &arena);
      let program = parser.parse();
      assert!(program.is_ok());
      assert_eq!(program.unwrap().statements.len(), $len);
    }
  };
  ($name:ident, $source:expr, $len:expr, $assertion:expr) => {
    #[test]
    fn $name() {
      let arena = ::bumpalo::Bump::new();
      let lexer = ::krama_lexer::lexer::Lexer::new($source);
      let mut parser = ::krama_parser::parser::Parser::new(lexer, &arena);
      let program = parser.parse();
      assert!(program.is_ok());
      let program = program.unwrap();
      assert_eq!(program.statements.len(), $len);
      let statement = &program.statements[0];
      $assertion(statement);
    }
  };
}

#[macro_export]
macro_rules! test_parser_error {
  ($name:ident, $source:expr, $assertion:expr) => {
    #[test]
    fn $name() {
      let arena = ::bumpalo::Bump::new();
      let lexer = ::krama_lexer::lexer::Lexer::new($source);
      let mut parser = ::krama_parser::parser::Parser::new(lexer, &arena);
      let program = parser.parse();
      assert!(program.is_err());
      let error = program.unwrap_err();
      $assertion(error);
    }
  };
}
