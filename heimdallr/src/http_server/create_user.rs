use actix_web::{Json, AsyncResponder, HttpResponse};

use futures::future::{Future, result};

use crate::http_server::{RequestWithState, FutureResponse};
use crate::db::CreateUser;

pub fn handler<'a>((data, request): (Json<CreateUser<'a>>, &RequestWithState)) -> FutureResponse {

  // state.database.send(data).from_err().and_then(|| Ok(HttpResponse::Ok())).responder()
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
