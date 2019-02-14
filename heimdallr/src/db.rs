mod schema;
pub use schema::*;

mod users;
pub use users::*;

use actix::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub struct Database {
  pub pool: Pool<ConnectionManager<PgConnection>>
}

impl Database {
  pub fn new(database: &str, username: &str, password: &str, host: &str, port: &str) -> Self {
    let database_url = format!(
      "postgres://{}:{}@{}:{}/{}",
      username,
      password,
      host,
      port,
      database
    );

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Database {
      pool: Pool::new(manager).expect("Error creating Postgres connection pool!")
    }
  }
}

unsafe impl Send for Database {}

impl Actor for Database {
  type Context = SyncContext<Self>;
}
