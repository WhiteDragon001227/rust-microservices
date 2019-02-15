use argonautica::{Hasher, Verifier};
use failure::Fallible;

/// Gets the password secret hash from an environment variable.
fn secret() -> Fallible<String> {
  match std::env::var("HEIMDALLR_SECRET") {
    Ok(val)  => Ok(val),
    Err(err) => Err(err.into())
  }
}

/// Hashes a password.
/// 
/// # Arguments
/// * `password` - The password to hash.
pub fn hash<S>(password: S) -> Fallible<String>
  where S: Into<String> {
  Ok(Hasher::default()
    .with_secret_key(secret()?)
    .with_password(password.into())
    .hash()?)
}

/// Verifies that a password matches a hash.
/// 
/// # Arguments
/// * `password` - The expected password.
/// * `hash`     - A hash created by `password::hash(...)`.
pub fn verify<S>(password: S, hash: S) -> Fallible<bool>
  where S: Into<String> {
  Ok(Verifier::default()
    .with_secret_key(secret()?)
    .with_password(password.into())
    .with_hash(hash.into())
    .verify()?)
}

#[cfg(test)]
mod tests {
  use speculate::speculate;
  use std::env;
  use super::*;

  speculate! {
    const TEST_PASSWORD: &str = "t0Ps3cr3T";

    it "hashes a password" {
      env::set_var("HEIMDALLR_SECRET", "Supercalifragilisticexpialidocious");
      let hashed_password = hash(TEST_PASSWORD);

      assert!(hashed_password.is_ok());
    }

    it "verifies a hashed password" {
      env::set_var("HEIMDALLR_SECRET", "Supercalifragilisticexpialidocious");
      let hashed_password = hash(TEST_PASSWORD).unwrap();

      assert!(verify(TEST_PASSWORD, &hashed_password).unwrap());
    }
  }
}
