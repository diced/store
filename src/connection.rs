use std::{net::SocketAddr, sync::Arc};

use bytes::BytesMut;
use dashmap::DashMap;
use log::debug;
use tokio::{
  io::{AsyncReadExt, AsyncWriteExt},
  net::TcpStream,
  sync::Mutex
};

use crate::{
  command::Command,
  value::{ResultValue, Value},
  StoreResult
};

#[derive(Debug, Clone)]
pub struct Connection {
  stream: Arc<Mutex<TcpStream>>,
  buffer: BytesMut
}

impl Connection {
  pub fn new(socket: Arc<Mutex<TcpStream>>) -> StoreResult<Self> {
    Ok(Self {
      stream: socket,
      buffer: BytesMut::with_capacity(1024 * 1024)
    })
  }

  pub async fn addr(&self) -> StoreResult<SocketAddr> {
    Ok(self.stream.lock().await.peer_addr()?)
  }

  pub async fn write_frame(&self, bytes: &[u8]) -> StoreResult<()> {
    let mut socket = self.stream.lock().await;

    socket.writable().await?;

    socket.write_all(&bytes).await?;

    Ok(())
  }

  pub async fn write_frame_command(&self, command: Command) -> StoreResult<()> {
    let bytes = bincode::serialize(&command)?;

    self.write_frame(&bytes).await?;

    Ok(())
  }

  pub async fn write_frame_result(&self, result: ResultValue) -> StoreResult<()> {
    let bytes = bincode::serialize(&result)?;

    self.write_frame(&bytes).await?;

    Ok(())
  }

  pub async fn read_frame(&mut self) -> StoreResult<usize> {
    let mut stream = self.stream.lock().await;

    let size = stream.read_buf(&mut self.buffer).await?;

    Ok(size)
  }

  pub async fn read_frame_result(&mut self) -> StoreResult<ResultValue> {
    let size = self.read_frame().await?;

    if size != 0 {
      let data: ResultValue = bincode::deserialize(&self.buffer)?;
      self.buffer.clear();

      Ok(data)
    } else {
      Ok(ResultValue::None)
    }
  }

  pub async fn read_frame_map(&mut self) -> StoreResult<DashMap<String, Value>> {
    let size = self.read_frame().await?;

    if size != 0 {
      let data: DashMap<String, Value> = bincode::deserialize(&self.buffer)?;
      self.buffer.clear();

      Ok(data)
    } else {
      Ok(DashMap::new())
    }
  }

  pub async fn read_frame_command(&mut self) -> StoreResult<Command> {
    let size = self.read_frame().await?;

    if size != 0 {
      let d: Command = bincode::deserialize(&self.buffer)?;
      self.buffer.clear();

      debug!("received command {}", d);
      Ok(d)
    } else {
      Ok(Command::None)
    }
  }

  pub async fn process_conn(&mut self, cache: DashMap<String, Value>) -> StoreResult<()> {
    loop {
      let cmd = self.read_frame_command().await.expect("couldn't read command");
      match cmd {
        Command::Get(key) => match cache.get(&key) {
          Some(v) => self.write_frame_result(v.value().into()).await?,
          None => self.write_frame_result(ResultValue::None).await?
        }
        Command::Set(key, value) => {
          cache.insert(key, value);
          self.write_frame_result(ResultValue::Ok).await?;
        }
        Command::Del(key) => {
          if cache.contains_key(&key) {
            cache.remove(&key);
            self.write_frame_result(ResultValue::Ok).await?;
          } else {
            self.write_frame_result(ResultValue::None).await?;
          }
        }

        Command::MapGet(map, key) => {
          match cache.get_mut(&map) {
            Some(map) => match map.value() {
              Value::Map(map) => match map.get(&key) {
                Some(v) => self.write_frame_result(ResultValue::OkValue(v.value().clone())).await?,
                None => self.write_frame_result(ResultValue::None).await?
              }
              Value::String(_)
               | Value::Integer(_)
               | Value::None => self.write_frame_result(ResultValue::None).await?
            }
            None => self.write_frame_result(ResultValue::None).await?
          }
        }
        Command::MapSet(map, key, val) => {
          match cache.get_mut(&map) {
            Some(map) => match map.value() {
              Value::Map(map) => {
                map.insert(key, val);
                self.write_frame_result(ResultValue::Ok).await?
              }
              Value::String(_)
               | Value::Integer(_)
               | Value::None => self.write_frame_result(ResultValue::None).await?
            }
            None => self.write_frame_result(ResultValue::None).await?
          }
        }
        Command::MapDel(map, key) => {
          match cache.get_mut(&map) {
            Some(map) => match map.value() {
              Value::Map(map) => match map.contains_key(&key) {
                true => {
                  map.remove(&key);
                  self.write_frame_result(ResultValue::Ok).await?
                },
                false => self.write_frame_result(ResultValue::None).await?
              }
              Value::String(_)
               | Value::Integer(_)
               | Value::None => self.write_frame_result(ResultValue::None).await?
            }
            None => self.write_frame_result(ResultValue::None).await?
          }
        }

        Command::GetAll => {
          let d = bincode::serialize(&cache)?;
          self.write_frame(&d).await?;
        }
        Command::None => {}
      };
    }
  }
}
