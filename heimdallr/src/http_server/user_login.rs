use actix_web::{AsyncResponder, HttpResponse, State, FutureResponse, Json};
use futures::future::Future;
use serde::Serialize;

use crate::{http_server::AppState, db::UserLogin};

/// TODO: Temporary struct for testing
#[derive(Serialize)]
pub struct LoginResponse {
  pub error: bool,
  pub message: Option<String>,
  pub token: Option<String>
}

/// HTTP handler for creating new users.
/// 
/// # Arguments
/// * `data`  - JSON payload.
/// * `state` - Application state (database pool, etc)
pub fn handler(data: Json<UserLogin>, state: State<AppState>) -> FutureResponse<HttpResponse> {
  state
    .database
    .send(data.into_inner())
    .from_err()
    .and_then(|res| match res {
      Ok(jwt)   => Ok(HttpResponse::Ok().json(LoginResponse{ error: false, message: None, token: Some(jwt) })),
      // TODO: This isn't the correct way to handle errors, but it works for now...
      Err(error) => Ok(HttpResponse::Ok().json(LoginResponse{ error: true, message: Some(error.as_fail().to_string()), token: None }))
  }).responder()
}
