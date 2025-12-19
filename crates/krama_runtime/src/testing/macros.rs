#[macro_export]
macro_rules! test_eval_ok {
  ($name:ident, $source:expr, $expected:expr) => {
    #[tokio::test]
    async fn $name() {
      let arena = ::bumpalo::Bump::new();
      let interpreter = $crate::Interpreter::new(&arena, None);
      let source = arena.alloc_str($source);
      let result = interpreter.eval(source).await.unwrap();
      assert_eq!(result, $expected);
    }
  };
}

#[macro_export]
macro_rules! test_eval_match {
  ($name:ident, $source:expr, $matcher:expr) => {
    #[tokio::test]
    async fn $name() {
      let arena = ::bumpalo::Bump::new();
      let interpreter = $crate::Interpreter::new(&arena, None);
      let source = arena.alloc_str($source);
      let result = interpreter.eval(source).await.unwrap();
      let matcher: fn(::krama_core::Object) -> bool = $matcher;
      assert!(matcher(result));
    }
  };
}

#[macro_export]
macro_rules! test_eval_err {
  ($name:ident, $source:expr, $expected:pat) => {
    #[tokio::test]
    async fn $name() {
      let arena = ::bumpalo::Bump::new();
      let interpreter = $crate::Interpreter::new(&arena, None);
      let source = arena.alloc_str($source);
      let result = interpreter.eval(source).await;
      assert!(matches!(result.unwrap_err().kind, $expected));
    }
  };

  ($name:ident, $source:expr) => {
    #[tokio::test]
    async fn $name() {
      let arena = ::bumpalo::Bump::new();
      let interpreter = $crate::Interpreter::new(&arena, None);
      let source = arena.alloc_str($source);
      let result = interpreter.eval(source).await;
      assert!(result.is_err());
    }
  };
}

#[macro_export]
macro_rules! test_eval_is_native_function {
  ($name:ident, $source:expr) => {
    #[tokio::test]
    async fn $name() {
      let arena = ::bumpalo::Bump::new();
      let interpreter = $crate::Interpreter::new(&arena, None);
      let source = arena.alloc_str($source);
      let result = interpreter.eval(source).await.unwrap();
      assert!(matches!(
        result,
        ::krama_core::Object::Function(::krama_core::Function::Native(_))
      ));
    }
  };
}

#[macro_export]
macro_rules! test_eval_is_module {
  ($name:ident, $source:expr, $expected:expr) => {
    #[tokio::test]
    async fn $name() {
      let arena = ::bumpalo::Bump::new();
      let interpreter = $crate::Interpreter::new(&arena, None);
      let source = arena.alloc_str($source);
      let result = interpreter.eval(source).await.unwrap();
      if let ::krama_core::Object::Scope(module) = result {
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
      let interpreter = $crate::Interpreter::new(&arena, Some(""));
      let input = arena.alloc_str($source);

      ::tokio::fs::write($filename, $file_content).await.unwrap();
      let evaluated = interpreter.eval(input).await.unwrap();
      ::tokio::fs::remove_file($filename).await.unwrap();

      assert_eq!(evaluated, $expected);
    }
  };
}
