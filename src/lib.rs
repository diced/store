pub mod client;
pub mod command;
pub mod connection;
pub mod server;
pub mod value;
pub mod config;

use std::error::Error;

pub type StoreResult<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;
