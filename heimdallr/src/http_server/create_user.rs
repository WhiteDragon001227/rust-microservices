use actix_web::{AsyncResponder, HttpResponse, State, FutureResponse, Json};
use futures::future::Future;
use serde::Serialize;

use crate::{http_server::AppState, db::CreateUser};

/// TODO: Temporary struct for testing
#[derive(Serialize)]
pub struct EpicFailure {
  pub error: bool,
  pub reason: String
}

/// HTTP handler for creating new users.
/// 
/// # Arguments
/// * `data`  - JSON payload.
/// * `state` - Application state (database pool, etc)
pub fn handler(data: Json<CreateUser>, state: State<AppState>) -> FutureResponse<HttpResponse> {
  state
    .database
    .send(data.into_inner())
    .from_err()
    .and_then(|res| match res {
      Ok(user)   => Ok(HttpResponse::Ok().json(user)),
      // TODO: This isn't the correct way to handle errors, but it works for now...
      Err(error) => Ok(HttpResponse::Ok().json(EpicFailure{ error: true, reason: error.as_fail().to_string() }))
  }).responder()
}
