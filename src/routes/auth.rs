use argon2::{Argon2, PasswordVerifier, PasswordHash};
use rocket::{serde::json::{Json, serde_json::json, Value}, response::status::Custom};

use crate::{models::{auth::Credentials, props::NotFoundUsernameProps}, repositories::users::UserRepository};

use super::{DbConn, not_found_user};

#[rocket::post("/login", format="json", data="<credentials>")]
pub async fn login(credentials: Json<Credentials>, db: DbConn) -> Result<Value, Custom<Value>> {
  db.run(move |c| {
    let cred = credentials.clone();
    UserRepository::find_by_username(c, &credentials.username)
      .map(|user| {
        let db_hash = PasswordHash::new(&user.password).unwrap();
        let argon = Argon2::default();
        if argon.verify_password(
          credentials.password.as_bytes(), &db_hash
        ).is_ok() {
          return json!("Success");
        }

        json!("Unauthorized")
      })
      .map_err(|e| {
        let params: NotFoundUsernameProps = NotFoundUsernameProps::new("login".to_string(), &cred.username, "users".to_string());
        not_found_user(e.into(), params)
      })
  }).await
}