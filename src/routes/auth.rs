use rocket::{serde::json::{Json, serde_json::json, Value}, response::status::Custom, http::Status};
use rocket_db_pools::{Connection, deadpool_redis::redis::AsyncCommands};

use crate::{models::{auth::Credentials, props::{UnAuthorixedProps, ServerErrorProps}}, repositories::auth::AuthRepository, functions::responses::{unauthorized, server_error}};

use super::{DbConn, CacheConn};

#[rocket::post("/login", format="json", data="<credentials>")]
pub async fn login(credentials: Json<Credentials>, db: DbConn, mut cache: Connection<CacheConn>) -> Result<Custom<Value>, Custom<Value>> {
  let username = credentials.username.clone();
  let username_cache = credentials.username.clone();
  let cred_db = credentials.clone().into_inner();

  let user = db.run(move |c| {
    AuthRepository::sign_in(c, cred_db)
      .map_err(|e| {
        match e {
            diesel::result::Error::NotFound => {
              let params: UnAuthorixedProps = UnAuthorixedProps::new("login".to_string(), &username);
              unauthorized(e.into(), params)
            },
            _ => {
              let params: ServerErrorProps = ServerErrorProps::new("login".to_string(), 0, "user".to_string());
              server_error(e.into(), params)
            }
        }
      })
  }).await?;

  let token = AuthRepository::authorize_user(&user, &credentials)
    .map_err(|e| {
      let message: String = format!("Wrong credentials for service login with username {} with message: {}", credentials.username, e);
      log::error!("{message}");
      Custom(Status::Unauthorized, json!("User or password not valid"))
    })?;

  cache.set_ex::<_, _, ()>(
    format!("sessions/{}", token),
    user.id,
    3*60*60
  )
  .await
  .map(|_| Custom(Status::Ok, json!({"token": token})))
  .map_err(|e| {
    let params: UnAuthorixedProps = UnAuthorixedProps::new("login".to_string(), &username_cache);
    unauthorized(e.into(), params)
  })
}