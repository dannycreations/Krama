use krama_core::{ErrorKind, Object};
use krama_runtime::{test_eval_err, test_eval_ok};

// --- Struct Initialization ---

test_eval_ok! {
  struct_init,
  r#"
    struct Point {
      pub x: f32,
      pub y: f32,
      
      pub fn new(x: f32, y: f32): this {
        this { x, y }
      }
    }
    
    const p = Point.new(1.0, 2.0)
    p.x
  "#,
  Object::Float(1.0)
}

// --- Struct Methods & "this" Context ---

test_eval_ok! {
  struct_methods,
  r#"
    struct Vec3 {
      x: f32,
      y: f32,
      z: f32,
      
      pub fn new(x: f32, y: f32, z: f32): this {
        this { x, y, z }
      }
      
      pub fn sum(this): f32 {
        this.x + this.y + this.z
      }
    }
    
    const v = Vec3.new(1.0, 2.0, 3.0)
    v.sum()
  "#,
  Object::Float(6.0)
}

// --- Default Field Values ---

test_eval_ok! {
  struct_defaults,
  r#"
    struct Config {
      pub port: i32 = 8080,
      pub host: str = "localhost",
      
      pub fn new(): this {
        this {}
      }
    }
    
    const c = Config.new()
    c.port
  "#,
  Object::Integer(8080)
}

// --- Visibility Control ---

test_eval_err! {
  struct_private_field,
  r#"
    struct Box {
      value: i32,
      
      pub fn new(v: i32): this {
        this { value: v }
      }
    }
    
    const b = Box.new(42)
    b.value
  "#,
  ErrorKind::TypeError(_)
}

test_eval_ok! {
  struct_private_field_internal_access,
  r#"
    struct Box {
      value: i32,
      pub fn new(v: i32): this { this { value: v } }
      pub fn get(this): i32 { this.value }
    }
    const b = Box.new(42)
    b.get()
  "#,
  Object::Integer(42)
}

// --- Recursion & Self-Reference ---

test_eval_ok! {
  struct_recursive_method,
  r#"
    struct Factorial {
      pub fn calc(n: i64): i64 {
        if (n <= 1) { 1 } else { n * this.calc(n - 1) }
      }
    }
    const f = Factorial {}
    f.calc(5)
  "#,
  Object::Integer(120)
}
