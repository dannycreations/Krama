use krama_core::{EnumInstance, Object};
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
  Object::Enum(Box::new(EnumInstance {
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
  Object::Enum(Box::new(EnumInstance {
    name: "Message".into(),
    variant: "Text".into(),
    fields: Some(vec![Object::String("hello".into())].into()),
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
  Object::Enum(Box::new(EnumInstance {
    name: "Point".into(),
    variant: "ThreeD".into(),
    fields: Some(
      vec![Object::Integer(1), Object::Integer(2), Object::Integer(3),].into(),
    ),
  }))
);
