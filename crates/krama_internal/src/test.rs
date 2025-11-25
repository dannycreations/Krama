#[allow(unused_imports)]
use futures::future::FutureExt;

#[macro_export]
macro_rules! resolve_future {
  ($result:expr) => {
    if let ::krama_core::object::Object::Future(future) = $result {
      if let Some(real_future) = future.take() {
        real_future.await.unwrap()
      } else {
        panic!("Future already resolved")
      }
    } else {
      $result
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
      assert_eq!(result, $expected);
    }
  };
}

#[macro_export]
macro_rules! test_eval_async {
  ($name:ident, $source:expr, $expected:expr) => {
    #[tokio::test]
    async fn $name() {
      let arena = ::bumpalo::Bump::new();
      let interpreter =
        ::krama_runtime::interpreter::Interpreter::new(&arena, None);
      let source = arena.alloc_str($source);
      let result = interpreter.eval(source).await.unwrap();
      let result = $crate::resolve_future!(result);
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
      let result = $crate::resolve_future!(result);
      assert!(matches!(result, ::krama_core::object::Object::NativeFn(_)));
    }
  };
}

#[macro_export]
macro_rules! test_eval_error {
  ($name:ident, $source:expr, $expected:expr) => {
    #[tokio::test]
    async fn $name() {
      let arena = ::bumpalo::Bump::new();
      let interpreter =
        ::krama_runtime::interpreter::Interpreter::new(&arena, None);
      let source = arena.alloc_str($source);
      let result = interpreter.eval(source).await;
      assert_eq!(result.unwrap_err().to_string(), $expected);
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
      let result = $crate::resolve_future!(result);
      if let ::krama_core::object::Object::Module(module) = result {
        assert_eq!(module.try_borrow().unwrap().name, $expected);
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

      tokio::fs::write($filename, $file_content).await.unwrap();
      let evaluated = interpreter.eval(input).await.unwrap();
      let evaluated = $crate::resolve_future!(evaluated);
      tokio::fs::remove_file($filename).await.unwrap();

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
      let program = parser.parse().unwrap();
      assert_eq!(program.statements.len(), $len);
    }
  };
  ($name:ident, $source:expr, $len:expr, $assertion:expr) => {
    #[test]
    fn $name() {
      let arena = ::bumpalo::Bump::new();
      let lexer = ::krama_lexer::lexer::Lexer::new($source);
      let mut parser = ::krama_parser::parser::Parser::new(lexer, &arena);
      let program = parser.parse().unwrap();
      assert_eq!(program.statements.len(), $len);
      let statement = &program.statements[0];
      $assertion(statement);
    }
  };
}
