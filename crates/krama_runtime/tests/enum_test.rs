use krama_core::ObjectKind;
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
  ObjectKind::Enum {
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
  ObjectKind::Enum {
    name: "Message",
    variant: "Text",
    fields: Some(&[ObjectKind::String("hello")]),
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
  ObjectKind::Enum {
    name: "Point",
    variant: "ThreeD",
    fields: Some(&[
      ObjectKind::Integer(1),
      ObjectKind::Integer(2),
      ObjectKind::Integer(3)
    ]),
  }
);
