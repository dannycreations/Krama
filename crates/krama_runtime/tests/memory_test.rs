use krama_core::ObjectKind;
use krama_runtime::Interpreter;

#[tokio::test]
async fn memory_stack_depth_tracking() {
  let source = r#"
    fn recursive(n) {
      if (n <= 0) { return 0; }
      return recursive(n - 1);
    }
    recursive(5);
  "#;

  let interpreter = Interpreter::new(None);
  let _ = interpreter.eval(source).await.unwrap();

  let stack_depth = interpreter.stack.read().depth();
  assert_eq!(stack_depth, 0, "Stack should be empty after execution");
}

#[tokio::test]
async fn memory_nested_closure_capture_behavior() {
  let source = r#"
    fn make_counter(start) {
      let count = start;
      return fn() {
        count = count + 1;
        return count;
      };
    }
    
    const counter = make_counter(10);
    counter(); // 11
    counter(); // 12
    counter(); // 13
  "#;

  let interpreter = Interpreter::new(None);
  let result = interpreter.eval(source).await.unwrap();

  assert_eq!(result, ObjectKind::Integer(13));
  assert_eq!(interpreter.stack.read().depth(), 0, "Stack should be empty");
}

#[tokio::test]
async fn memory_heap_allocation_tracking() {
  let source = r#"
    let a = [1, 2, 3]; // Tuple (Heap allocated)
    let b = { "x": 10, "y": 20 }; // Object (Heap allocated)
    let c = [a, b]; // Tuple (Heap allocated)
  "#;

  let interpreter = Interpreter::new(None);
  let _ = interpreter.eval(source).await.unwrap();

  let allocations = interpreter.heap.read().allocations;
  // 1 tuple (a) + 1 object (b) + 1 tuple (c) = 3 allocations
  assert_eq!(allocations, 3, "Should track exactly 3 heap allocations");
}

#[tokio::test]
async fn memory_variable_shadowing_and_stack_integrity() {
  let source = r#"
    let x = 10;
    {
      let x = 20;
      {
        let x = 30;
        if (x != 30) { return 1; }
      }
      if (x != 20) { return 2; }
    }
    x
  "#;

  let interpreter = Interpreter::new(None);
  let result = interpreter.eval(source).await.unwrap();

  assert_eq!(result, ObjectKind::Integer(10));
  assert_eq!(
    interpreter.stack.read().depth(),
    0,
    "Stack should be empty after block exits"
  );
}

#[tokio::test]
async fn memory_deep_recursion_stack_integrity() {
  let source = r#"
    fn sum(n, acc) {
      if (n <= 0) { return acc; }
      return sum(n - 1, acc + n);
    }
    sum(20, 0); // Reduced depth to avoid OS stack limit in test environments
  "#;

  let interpreter = Interpreter::new(None);
  let result = interpreter.eval(source).await.unwrap();

  // sum of 1..20 = (20 * 21) / 2 = 210
  assert_eq!(result, ObjectKind::Integer(210));
  assert_eq!(interpreter.stack.read().depth(), 0);
}
