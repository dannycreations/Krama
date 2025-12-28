#[macro_export]
macro_rules! test_eval_ok {
  ($name:ident, $source:expr, $expected:expr) => {
    #[tokio::test]
    async fn $name() {
      let interpreter = $crate::Interpreter::new(None);
      let result = interpreter.eval($source).await.unwrap();
      assert_eq!(result, $expected);
    }
  };
}

#[macro_export]
macro_rules! test_eval_match {
  ($name:ident, $source:expr, $expected:pat $(if $guard:expr)? ) => {
    #[tokio::test]
    async fn $name() {
      let interpreter = $crate::Interpreter::new(None);
      let result = interpreter.eval($source).await.unwrap();
      assert!(matches!(result, $expected $(if $guard)?));
    }
  };
}

#[macro_export]
macro_rules! test_eval_err {
  ($name:ident, $source:expr, $expected:pat) => {
    #[tokio::test]
    async fn $name() {
      let interpreter = $crate::Interpreter::new(None);
      let result = interpreter.eval($source).await;
      assert!(matches!(result.unwrap_err().kind, $expected));
    }
  };

  ($name:ident, $source:expr) => {
    #[tokio::test]
    async fn $name() {
      let interpreter = $crate::Interpreter::new(None);
      let result = interpreter.eval($source).await;
      assert!(result.is_err());
    }
  };
}

#[macro_export]
macro_rules! test_eval_is_native_function {
  ($name:ident, $source:expr) => {
    #[tokio::test]
    async fn $name() {
      use krama_core::{Function, Object};
      let interpreter = $crate::Interpreter::new(None);
      let result = interpreter.eval($source).await.unwrap();
      assert!(matches!(result, Object::Function(Function::Native(_))));
    }
  };
}

#[macro_export]
macro_rules! test_eval_is_module {
  ($name:ident, $source:expr, $expected:expr) => {
    #[tokio::test]
    async fn $name() {
      use krama_core::Object;
      let interpreter = $crate::Interpreter::new(None);
      let result = interpreter.eval($source).await.unwrap();
      if let Object::Scope(module) = result {
        assert_eq!(module.read().name.as_deref(), $expected);
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
      use tokio::fs;
      let interpreter = $crate::Interpreter::new(Some("".to_string()));
      fs::write($filename, $file_content).await.unwrap();
      let evaluated = interpreter.eval($source).await.unwrap();
      fs::remove_file($filename).await.unwrap();
      assert_eq!(evaluated, $expected);
    }
  };
}
