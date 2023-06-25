use rocket::{serde::json::{Json, serde_json::json, Value}, response::status::Custom, http::Status};
use rocket_db_pools::{Connection, deadpool_redis::redis::AsyncCommands};

use crate::{models::{auth::Credentials, props::{UnAuthorixedProps, ServerErrorProps, NotFoundProps}}, repositories::{auth::AuthRepository, roles::RoleRepository, people::PersonRepository}, functions::responses::{unauthorized, server_error, not_found}, commands::users::load_db_connection};

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

  let mut conn1 = load_db_connection();
  let roles = RoleRepository::find_by_user(&mut conn1, &user)
    .map_err(|e| {
      let params: NotFoundProps = NotFoundProps::new("login".to_string(), user.id, "roles".to_string());
      not_found(e.into(), params)
    })?;

  let mut conn2 = load_db_connection();
  let person = PersonRepository::find_by_user(&mut conn2, user.id)
    .map_err(|e| {
      let params: NotFoundProps = NotFoundProps::new("login".to_string(), user.id, "people".to_string());
      not_found(e.into(), params)
    })?;

  let mut conn3 = load_db_connection();
  let parents = PersonRepository::find_parents(&mut conn3, user.id)
    .map_err(|e| {
      let params: NotFoundProps = NotFoundProps::new("login".to_string(), user.id, "people".to_string());
      not_found(e.into(), params)
    })?;

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
  .map(|_| Custom(Status::Ok, json!({"token": token, "user": user, "person": person, "parents": parents, "roles": roles})))
  .map_err(|e| {
    let params: UnAuthorixedProps = UnAuthorixedProps::new("login".to_string(), &username_cache);
    unauthorized(e.into(), params)
  })
}