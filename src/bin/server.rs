use log::{info, LevelFilter};
use simple_logger::SimpleLogger;
use store::{server::Server, StoreResult};

#[tokio::main]
async fn main() -> StoreResult<()> {
  SimpleLogger::new()
    .with_module_level("mio", LevelFilter::Off)
    .init()?;

  let mut server = Server::new("store.toml".into()).await?;
  info!("started store server on {}", server.addr());

  server.run().await?;

  Ok(())
}
