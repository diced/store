use dashmap::DashMap;
use store::{client::Client, value::Value, StoreResult};

#[tokio::main]
async fn main() -> StoreResult<()> {
  let mut client = Client::new("127.0.0.1:6767").await?;

  Ok(())
}
