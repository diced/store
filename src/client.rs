use std::sync::Arc;

use dashmap::DashMap;
use tokio::{
  net::{TcpStream, ToSocketAddrs},
  sync::Mutex
};

use crate::{
  command::Command,
  connection::Connection,
  value::{ResultValue, Value},
  StoreResult
};

#[derive(Debug)]
pub struct Client {
  conn: Connection
}

impl Client {
  pub async fn new<A: ToSocketAddrs>(a: A) -> StoreResult<Self> {
    let stream = Arc::new(Mutex::new(TcpStream::connect(a).await.unwrap()));

    Ok(Self {
      conn: Connection::new(stream)?
    })
  }

  pub async fn get(&mut self, key: impl Into<String>) -> StoreResult<ResultValue> {
    self
      .conn
      .write_frame_command(Command::Get(key.into()))
      .await?;
    let res = self.conn.read_frame_result().await?;
    Ok(res)
  }

  pub async fn set(
    &mut self,
    key: impl Into<String>,
    val: impl Into<Value>
  ) -> StoreResult<ResultValue> {
    self
      .conn
      .write_frame_command(Command::Set(key.into(), val.into()))
      .await?;
    let res = self.conn.read_frame_result().await?;
    Ok(res)
  }

  pub async fn del(&mut self, key: impl Into<String>) -> StoreResult<ResultValue> {
    self.conn.write_frame_command(Command::Del(key.into())).await?;
    let res = self.conn.read_frame_result().await?;
    Ok(res)
  }

  pub async fn map_set(&mut self, map: impl Into<String>, key: impl Into<String>, val: impl Into<Value>) -> StoreResult<ResultValue> {
    self.conn.write_frame_command(Command::MapSet(map.into(), key.into(), val.into())).await?;
    let res = self.conn.read_frame_result().await?;
    Ok(res)
  }

  pub async fn map_get(&mut self, map: impl Into<String>, key: impl Into<String>) -> StoreResult<ResultValue> {
    self.conn.write_frame_command(Command::MapGet(map.into(), key.into())).await?;
    let res = self.conn.read_frame_result().await?;
    Ok(res)
  }

  pub async fn map_del(&mut self, map: impl Into<String>, key: impl Into<String>) -> StoreResult<ResultValue> {
    self.conn.write_frame_command(Command::MapDel(map.into(), key.into())).await?;
    let res = self.conn.read_frame_result().await?;
    Ok(res)
  }

  pub async fn get_all(&mut self) -> StoreResult<DashMap<String, Value>> {
    self.conn.write_frame_command(Command::GetAll).await?;
    let res = self.conn.read_frame_map().await?;
    Ok(res)
  }
}
