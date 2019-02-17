use super::schema::users;

use actix::prelude::*;
use diesel::prelude::*;
use chrono::prelude::*;
use failure::{Fallible, format_err};
use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Queryable, Serialize, Debug)]
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
#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct CreateUser {
  #[validate(email(message = "Email %s is not valid."))]
  pub email: String,

  #[validate(length(min = "4"))]
  pub password: String
}

impl Message for CreateUser {
  type Result = Fallible<User>;
}

/// Implements a handler for creating new users in the database.
impl Handler<CreateUser> for super::Database {
  type Result = Fallible<User>;

  fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
    use super::schema::users::dsl::*;
    use crate::password;
    use diesel::dsl::*;

    // Grab a connection from the pool
    let connection = self.pool.get()?;

    // Check if the user already exists
    match select(
      exists(users.filter(email.eq(&msg.email)))
    ).get_result::<bool>(&connection) {
      Ok(true) => { return Err(format_err!("User with this email address already exists.")) },
      _ => {}
    }

    // Create a new user to insert into the database
    let new_user = NewUser {
      email: &msg.email,
      password_digest: &password::hash(msg.password)?
    };

    Ok(diesel::insert_into(users)
      .values(&new_user)
      .get_result::<User>(&connection)?
    )
  }
}



#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct UserLogin {
#[validate(email(message = "Email %s is not valid."))]
  pub email: String,
  pub password: String
}

impl Message for UserLogin {
  type Result = Fallible<String>;
}

#[derive(Serialize, Deserialize, Debug)]
struct JwtClaims {
  sub: String,
  iat: i64,
  exp: i64
}

/// Implements a handler for creating new users in the database.
impl Handler<UserLogin> for super::Database {
  type Result = Fallible<String>;

  fn handle(&mut self, msg: UserLogin, _: &mut Self::Context) -> Self::Result {
    use jsonwebtoken::{encode, Header, Algorithm};
    use super::schema::users::dsl::*;
    use chrono::prelude::*;
    use crate::password;

    // Grab a connection from the pool
    let connection = self.pool.get()?;

    let user = users.filter(email.eq(&msg.email)).first::<User>(&connection)?;
    if !password::verify(msg.password, user.password_digest)? {
      //return Err(UserLoginError::BadUsernameOrPassword);
    }

    let mut header = Header::default();
    header.alg = Algorithm::HS512;

    let expiration = Utc::now();
    let payload = JwtClaims {
      sub: user.uuid.to_string(),
      iat: expiration.timestamp(),
      exp: expiration.timestamp() + 3600
    };

    Ok(encode(&header, &payload, "supercalifragilisticexpialidocious".as_ref())?)
  }
}
