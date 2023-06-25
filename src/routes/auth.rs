use rocket::{serde::json::{Json, serde_json::json, Value}, response::status::Custom, http::Status};
use rocket_db_pools::Connection;

use crate::{models::{auth::Credentials, props::UnAuthorixedProps}, repositories::auth::AuthRepository, functions::responses::unauthorized};

use super::{DbConn, CacheConn};

#[rocket::post("/login", format="json", data="<credentials>")]
pub async fn login(credentials: Json<Credentials>, db: DbConn, cache: Connection<CacheConn>) -> Result<Custom<Value>, Custom<Value>> {
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
        let params: UnAuthorixedProps = UnAuthorixedProps::new("login".to_string(), &cred.username);
        unauthorized(e.into(), params)
      })
  }).await
}