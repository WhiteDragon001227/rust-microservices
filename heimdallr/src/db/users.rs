use super::schema::users;

use actix::prelude::*;
use diesel::prelude::*;
use chrono::prelude::*;
use failure::{Fallible, format_err};
use validator::Validate;

#[derive(Queryable, Debug)]
pub struct User {
  pub id: i32,
  pub uuid: uuid::Uuid,
  email: String,
  password_digest: String,
  created_at: NaiveDateTime,
  updated_at: NaiveDateTime
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
  pub email: &'a str,
  pub password_digest: &'a str
}

/// Used to create a new user in the database.
#[derive(Validate, Debug)]
pub struct CreateUser<'a> {
  #[validate(email(message = "Email %s is not valid."))]
  pub email: &'a str,

  #[validate(length(min = "4"))]
  pub password: &'a str
}

impl<'a> Message for CreateUser<'a> {
  type Result = Fallible<User>;
}

/// Implements a handler for creating new users in the database.
impl<'a> Handler<CreateUser<'a>> for super::Database {
  type Result = Fallible<User>;

  fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
    use super::schema::users::dsl::*;
    use crate::password;
    use diesel::dsl::*;

    // Grab a connection from the pool
    let connection = self.pool.get()?;

    // Check if the user already exists
    match select(
      exists(users.filter(email.eq(msg.email)))
    ).get_result::<bool>(&connection) {
      Ok(true) => { return Err(format_err!("User with this email address already exists.")) },
      _ => {}
    }

    // Create a new user to insert into the database
    let new_user = NewUser {
      email: msg.email,
      password_digest: &password::hash(msg.password)?
    };

    Ok(diesel::insert_into(users)
      .values(&new_user)
      .get_result::<User>(&connection)?
    )
  }
}
