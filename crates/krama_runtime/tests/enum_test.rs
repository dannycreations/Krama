use krama_core::Object;
use krama_runtime::test_eval_ok;

test_eval_ok!(
  eval_simple_enum,
  r#"
    enum Status {
      Active,
      Inactive
    }

    Status.Active
  "#,
  Object::Enum {
    name: "Status",
    variant: "Active",
    fields: None,
  }
);

test_eval_ok!(
  eval_enum_with_fields,
  r#"
    enum Message {
      Text(str),
      Quit
    }

    Message.Text("hello")
  "#,
  Object::Enum {
    name: "Message",
    variant: "Text",
    fields: Some(&[Object::String("hello")]),
  }
);

test_eval_ok!(
  eval_enum_with_multiple_fields,
  r#"
    enum Point {
      TwoD(i64, i64),
      ThreeD(i64, i64, i64)
    }

    Point.ThreeD(1, 2, 3)
  "#,
  Object::Enum {
    name: "Point",
    variant: "ThreeD",
    fields: Some(&[Object::Integer(1), Object::Integer(2), Object::Integer(3)]),
  }
);
