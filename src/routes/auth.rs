use rocket::{serde::json::{Json, serde_json::json, Value}, response::status::Custom, http::Status};

use crate::{models::{auth::Credentials, props::NotFoundUsernameProps}, repositories::auth::AuthRepository, functions::responses::not_found_user};

use super::DbConn;

#[rocket::post("/login", format="json", data="<credentials>")]
pub async fn login(credentials: Json<Credentials>, db: DbConn) -> Result<Custom<Value>, Custom<Value>> {
  db.run(move |c| {
    let cred = credentials.clone();

    AuthRepository::sign_in(c, credentials.clone().into_inner())
      .map(|user| {
        if let Ok(token) = AuthRepository::authorize_user(&user, &credentials) {
          return Custom(Status::Ok, json!(token))
        }

        Custom(Status::Unauthorized, json!("Unauthorized"))
      })
      .map_err(|e| {
        let params: NotFoundUsernameProps = NotFoundUsernameProps::new("login".to_string(), &cred.username, "users".to_string());
        not_found_user(e.into(), params)
      })
  }).await
}