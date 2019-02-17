mod create_user;

use openssl::ssl::{SslMethod, SslAcceptor, SslAcceptorBuilder, SslFiletype};

use actix::{prelude::*, SystemRunner};
use actix_web::{
  middleware::{self, cors::Cors},
  http::{Method, header::CONTENT_TYPE},
  App, server
};

use failure::Fallible;

use crate::settings::{Settings, TLSConfig};
use crate::db::Database;

/// Contains the application state which gets passed around to each handler.
pub struct AppState {
  pub database: Addr<Database>
}

/// HTTP Server object.
pub struct Server {
  runner: SystemRunner
}

impl Server {
  /// Creates a new HTTP server using settings.
  /// 
  /// # Arguments
  /// * `settings` - Settings to use.
  pub fn from_settings(settings: &Settings) -> Fallible<Server> {
    let runner = actix::System::new("heimdallr_http");

    // Initialize the database connection
    let db_threads: usize = settings.database.pool.unwrap_or(5);
    let database = Database::from_settings(&settings)?;
    let db_addr  = SyncArbiter::start(db_threads, move || database.clone());
    
    let server = server::new(move || {
      App::with_state(AppState { database: db_addr.clone() })
        .middleware(middleware::Logger::default())
        .configure(|app| {
          Cors::for_app(app)
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_header(CONTENT_TYPE)
            .max_age(3600)
            .resource("/users/create", |r| {
              r.method(Method::POST).with_async(create_user::handler)
            })
            .register()
        })
    });

    if settings.inbound_listener.tls.enabled {
      server.bind_ssl(
        &settings.inbound_listener.address,
        Self::build_tls(&settings.inbound_listener.tls)?
      )?.start();
    }
    else {
      server.bind(&settings.inbound_listener.address)?.start();
    }

    Ok(Server{ runner })
  }

  /// Starts the HTTP server.
  pub fn start(self) -> i32 {
    self.runner.run()
  }

  /// Creates an SSL Acceptor object.
  /// 
  /// # Arguments
  /// * `tls` - TLS configuration settings.
  fn build_tls(tls: &TLSConfig) -> Fallible<SslAcceptorBuilder> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    builder.set_private_key_file(&tls.private_key, SslFiletype::PEM)?;
    builder.set_certificate_chain_file(&tls.cert)?;
    Ok(builder)
  }
}
