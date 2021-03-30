use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use dashmap::DashMap;
use log::info;
use tokio::{
  net::{TcpListener, TcpStream},
  sync::Mutex
};

use crate::{StoreResult, config::{Config, ParsedConfig}, connection::Connection, value::Value};

#[derive(Debug)]
pub struct Server {
  listener: TcpListener,
  cache: DashMap<String, Value>,
  config: Config
}

impl Server {
  pub async fn new(config_path: PathBuf) -> StoreResult<Self> {
    let config = ParsedConfig::parse(config_path).await?.to_config()?;

    Ok(Self {
      config: config.clone(),
      listener: TcpListener::bind(config.host).await?,
      cache: DashMap::new()
    })
  }

  pub fn addr(&self) -> SocketAddr {
    self.config.host
  }

  async fn accept(&mut self) -> StoreResult<(TcpStream, SocketAddr)> {
    loop {
      match self.listener.accept().await {
        Ok(s) => return Ok(s),
        Err(err) => return Err(err.into())
      }
    }
  }

  pub async fn run(&mut self) -> StoreResult<()> {
    loop {
      let (stream, addr) = self.accept().await.expect("rip");
      info!("incoming client from {:?}", addr);

      let conn = Connection::new(Arc::new(Mutex::new(stream)))?;

      let cache = self.cache.clone();
      let mut conn = conn.clone();
      tokio::spawn(async move {
        conn.process_conn(cache).await.expect("couldn't process connection");
      });
    }
  }
}
