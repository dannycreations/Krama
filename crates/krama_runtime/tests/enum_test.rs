use krama_core::{EnumInstance, ObjectKind};
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
  ObjectKind::Enum(Box::new(EnumInstance {
    name: "Status".into(),
    variant: "Active".into(),
    fields: None,
  }))
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
  ObjectKind::Enum(Box::new(EnumInstance {
    name: "Message".into(),
    variant: "Text".into(),
    fields: Some(vec![ObjectKind::String("hello".into())].into()),
  }))
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
  ObjectKind::Enum(Box::new(EnumInstance {
    name: "Point".into(),
    variant: "ThreeD".into(),
    fields: Some(
      vec![
        ObjectKind::Integer(1),
        ObjectKind::Integer(2),
        ObjectKind::Integer(3),
      ]
      .into(),
    ),
  }))
);
