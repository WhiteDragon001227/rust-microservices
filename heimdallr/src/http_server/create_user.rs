use actix_web::{Json, Error, State, AsyncResponder, HttpResponse};

use futures::future::{Future};

use crate::http_server::{RequestWithState, FutureResponse};
use crate::db::CreateUser;

// pub fn handler<'a>((data, state): (Json<CreateUser<'a>>, State<AppState>)) -> impl Future<Item = HttpResponse, Error = Error> {
pub fn handler<'a>(request: &RequestWithState) -> FutureResponse {

  request
    .state()
    .database
    .send(CreateUser{ email: "nater540@gmail.com", password: "magical1" })
    // .send(data.into_inner())
    .from_err()
    .and_then(|result| {
      Ok(HttpResponse::Ok().body("hello world"))
  }).responder()
}
