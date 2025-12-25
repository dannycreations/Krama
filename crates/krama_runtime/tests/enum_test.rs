use krama_core::ObjectKind;
use krama_runtime::test_eval_ok;

test_eval_ok!(
  enum_simple,
  r#"
    enum Status {
      Active,
      Inactive
    }

    Status.Active
  "#,
  ObjectKind::Enum {
    name: "Status".to_string(),
    variant: "Active".to_string(),
    fields: None,
  }
);

test_eval_ok!(
  enum_with_fields,
  r#"
    enum Message {
      Text(str),
      Quit
    }

    Message.Text("hello")
  "#,
  ObjectKind::Enum {
    name: "Message".to_string(),
    variant: "Text".to_string(),
    fields: Some(vec![ObjectKind::String("hello".to_string())]),
  }
);

test_eval_ok!(
  enum_with_multiple_fields,
  r#"
    enum Point {
      TwoD(i64, i64),
      ThreeD(i64, i64, i64)
    }

    Point.ThreeD(1, 2, 3)
  "#,
  ObjectKind::Enum {
    name: "Point".to_string(),
    variant: "ThreeD".to_string(),
    fields: Some(vec![
      ObjectKind::Integer(1),
      ObjectKind::Integer(2),
      ObjectKind::Integer(3)
    ]),
  }
);
