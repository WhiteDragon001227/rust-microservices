use config::{ConfigError, Config, File, Environment};
use std::net::SocketAddr;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
  pub inbound_listener: Listener,
  pub database: Database
}

#[derive(Debug, Deserialize)]
pub struct Database {
  pub name: String,
  pub host: String,
  pub port: Option<i32>,
  pub username: String,
  pub password: String,
  pub pool: Option<usize>
}

#[derive(Debug, Deserialize)]
pub struct Listener {
  pub address: SocketAddr,
  pub backlog: Option<i16>,
  pub workers: Option<i16>,

  #[serde(default)]
  pub tls: TLSConfig
}

#[derive(Debug, Default, Deserialize)]
pub struct TLSConfig {
  pub enabled: bool,
  pub private_key: String,
  pub cert: String
}

impl Settings {
  pub fn new(config_path: &str) -> Result<Self, ConfigError> {
    let mut cfg = Config::new();

    cfg.merge(File::with_name(config_path).required(false))?;
    cfg.merge(Environment::with_prefix("heimdallr").separator("_"))?;

    // Deserialize and freeze the entire configuration
    cfg.try_into()
  }
}
