mod integer;
mod string;

use dashmap::DashMap;
pub use integer::IntegerValue;
pub use string::StringValue;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Value {
  String(StringValue),
  Integer(IntegerValue),
  Map(DashMap<String, Value>),
  None
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ResultValue {
  OkValue(Value),
  Ok,
  None
}

impl Into<ResultValue> for &Value {
  fn into(self) -> ResultValue {
    match self.clone() {
      Value::String(v) => ResultValue::OkValue(Value::String(v)),
      Value::Integer(v) => ResultValue::OkValue(Value::Integer(v)),
      Value::Map(v) => ResultValue::OkValue(Value::Map(v)),
      Value::None => ResultValue::None
    }
  }
}
