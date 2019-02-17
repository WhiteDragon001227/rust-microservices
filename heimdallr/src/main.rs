#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate validator_derive;

mod db;
mod logging;
mod settings;
mod password;
mod http_server;

use settings::Settings;
use http_server::Server;

use std::process::exit;
use failure::Fallible;
use clap::{App, Arg};

fn main() -> Fallible<()> {
  logging::init()?;

  let arguments = App::new("Heimdallr")
    .about("API Authentication Service")
    .version("1.0")
    .arg(
      Arg::with_name("config")
        .long("config")
        .short("c")
        .value_name("FILE")
        .help("Sets a custom config file")
        .takes_value(true)
    ).get_matches();

  // Figure out what config file to load
  let cwd = ::std::env::current_dir()?;
  let default_config = format!("{}/config.yaml", cwd.display());
  let config_file    = arguments.value_of("config").unwrap_or(&default_config);

  let settings = Settings::new(config_file)?;

  let server = Server::from_settings(&settings)?;
  exit(server.start());
}
