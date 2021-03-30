use std::{net::SocketAddr, path::PathBuf};

use log::{debug, info};
use tokio::fs::read_to_string;
use serde::{Serialize, Deserialize};

use crate::StoreResult;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
  pub host: SocketAddr,
  pub max_keys: u64
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ParsedConfig {
  pub host: Option<String>,
  pub max_keys: Option<u64>
}

impl ParsedConfig {
  pub async fn parse(path: PathBuf) -> StoreResult<Self> {
    debug!("trying to read config from {}", path.display());
    let data = read_to_string(path).await?;
    info!("read config");

    Ok(toml::from_str(&data).expect("couldn't read config"))
  }

  pub fn to_config(&self) -> StoreResult<Config> {
    Ok(Config {
      host: self.host.as_ref().unwrap_or(&"127.0.0.1:6767".to_string()).parse::<SocketAddr>().expect("host is not"),
      max_keys: self.max_keys.unwrap_or(128_u64)
    })
  }
}
