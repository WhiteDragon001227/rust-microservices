use argonautica::{Hasher, Verifier, Error};
use failure::Fallible;
use log::error;

/// Gets the password secret hash from an environment variable.
fn secret() -> String {
  match std::env::var("HEIMDALLR_SECRET") {
    Ok(secret) => secret,
    Err(err) => {
      error!("Password secret must be set via `HEIMDALLR_SECRET` environment variable! ({})", err);
      String::default()
    }
  }
}

/// Hashes a password.
/// 
/// # Arguments
/// * `password` - The password to hash.
pub fn hash(password: &str) -> Result<String, Error> {
  Hasher::default()
    .with_secret_key(secret())
    .with_password(password)
    .hash()
}

/// Verifies that a password matches a hash.
/// 
/// # Arguments
/// * `password` - The expected password.
/// * `hash`     - A hash created by `password::hash(...)`.
pub fn verify(password: &str, hash: &str) -> Result<bool, Error> {
  Verifier::default()
    .with_secret_key(secret())
    .with_password(password)
    .with_hash(hash)
    .verify()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn hashes_string() -> Result<(), String> {
    Ok(())
  }
}
