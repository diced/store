use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct StringValue {
  pub value: String
}

impl StringValue {
  pub fn new(val: impl Into<String>) -> Self {
    Self { value: val.into() }
  }
}

impl From<&str> for StringValue {
  fn from(value: &str) -> Self {
    Self {
      value: value.to_string()
    }
  }
}

impl fmt::Display for StringValue {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.value)
  }
}

mod tests {
  #[test]
  fn string_from() {
    use super::StringValue;

    let my_val_from = StringValue::from("test from");
    assert_eq!(
      my_val_from,
      StringValue {
        value: "test from".to_string()
      }
    );
  }

  #[test]
  fn string_into() {
    use super::StringValue;

    let my_val_into: StringValue = "test into".into();
    assert_eq!(
      my_val_into,
      StringValue {
        value: "test into".to_string()
      }
    );
  }

  #[test]
  fn string_fmt() {
    use super::StringValue;

    let my_val = StringValue::new("test");

    println!("{}", my_val);
  }
}
