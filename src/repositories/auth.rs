use argon2::{Argon2, PasswordVerifier, PasswordHasher, PasswordHash, password_hash::{Error, SaltString}};
use diesel::{PgConnection, QueryResult};
use rand::{Rng, distributions::Alphanumeric, rngs::OsRng};
use crate::models::{users::User, auth::Credentials};

use super::users::UserRepository;

pub struct AuthRepository;

impl AuthRepository {
  pub fn sign_in(c: &mut PgConnection, credentials: Credentials) -> QueryResult<User> {
    UserRepository::find_by_username(c, &credentials.username)
  }

  pub fn authorize_user(user: &User, credentials: &Credentials) -> Result<String, Error> {
    let db_hash = PasswordHash::new(&user.password)?;
    let argon = Argon2::default();
    argon.verify_password(
      credentials.password.as_bytes(), &db_hash
    )?;

    Ok(
      rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(128)
        .map(char::from)
        .collect::<String>()
    )
  }

  pub fn hash_password(password: String) -> Result<String, Error> {
    let salt = SaltString::generate(OsRng);
    let argon = Argon2::default();
    let password_hash = argon.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
  }
}