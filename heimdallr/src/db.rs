mod schema;
pub use schema::*;

mod users;
pub use users::*;

use crate::settings::Settings;

use actix::prelude::*;
use failure::Fallible;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

/// Database object.
#[derive(Clone)]
pub struct Database {
  pub pool: Pool<ConnectionManager<PgConnection>>
}

impl Database {

  /// Creates a database connection using settings.
  /// 
  /// # Arguments
  /// * `settings` - Settings to use.
  pub fn from_settings(settings: &Settings) -> Fallible<Self> {
    let port: i32 = settings.database.port.unwrap_or(5432);

    Self::new(
      settings.database.name.to_owned(),
      settings.database.username.to_owned(),
      settings.database.password.to_owned(),
      settings.database.host.to_owned(),
      &port
    )
  }

  /// Creates a new database connection.
  /// 
  /// # Arguments
  /// * `database` - Database name to connect to.
  /// * `username` - Username to use.
  /// * `password` - Password for above username.
  /// * `host`     - Database server host.
  /// * `port`     - Database server port.
  pub fn new<D, U, P, H>(database: D, username: U, password: P, host: H, port: &i32) -> Fallible<Self>
    where
      D:  Into<String>,
      U:  Into<String>,
      P:  Into<String>,
      H:  Into<String> {

    // Format a postgres connection string
    let database_url = format!(
      "postgres://{}:{}@{}:{}/{}",
      username.into(),
      password.into(),
      host.into(),
      port,
      database.into()
    );

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool    = Pool::builder().build(manager)?;
    Ok(Database { pool })
  }
}

unsafe impl Send for Database {}

impl Actor for Database {
  type Context = SyncContext<Self>;
}
