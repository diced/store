use std::fmt;

use crate::value::Value;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Command {
  Get(String),
  Set(String, Value),
  Del(String),

  MapGet(String, String),
  MapSet(String, String, Value),
  MapDel(String, String),

  GetAll,
  None
}

impl fmt::Display for Command {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let readable_name = match self {
      Command::Get(_) => "Get",
      Command::Set(_, _) => "Set",
      Command::Del(_) => "Del",

      Command::MapGet(_, _) => "MapGet",
      Command::MapSet(_, _, _) => "MapSet",
      Command::MapDel(_, _) => "MapDel",

      Command::GetAll => "GetAll",
      Command::None => "None"
    };

    write!(f, "{}", readable_name)
  }
}
