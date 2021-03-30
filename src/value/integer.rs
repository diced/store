use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct IntegerValue {
  pub value: u64
}

impl IntegerValue {
  pub fn new(val: impl Into<u64>) -> Self {
    Self { value: val.into() }
  }
}

impl From<i32> for IntegerValue {
  fn from(value: i32) -> Self {
    Self {
      value: value as u64
    }
  }
}

impl fmt::Display for IntegerValue {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.value)
  }
}

mod tests {
  #[test]
  fn string_from() {
    use super::IntegerValue;

    let my_val_from = IntegerValue::from(69);
    assert_eq!(my_val_from, IntegerValue { value: 69_u64 });
  }

  #[test]
  fn string_into() {
    use super::IntegerValue;

    let my_val_into: IntegerValue = 420.into();
    assert_eq!(my_val_into, IntegerValue { value: 420_u64 });
  }

  #[test]
  fn string_fmt() {
    use super::IntegerValue;

    let my_val = IntegerValue::new(2_u64);

    println!("{}", my_val);
  }
}
