use rocket::{serde::json::{Json, serde_json::json, Value}, response::status::Custom, http::Status};
use rocket_db_pools::{Connection, deadpool_redis::redis::AsyncCommands};

use crate::{models::{auth::Credentials, props::{UnAuthorixedProps, ServerErrorProps, NotFoundProps}, signin::{UserWithRoles, SignIn}, roles::Role}, repositories::{auth::AuthRepository, roles::RoleRepository, people::PersonRepository, users::UserRepository}, functions::responses::{unauthorized, server_error, not_found}, commands::users::load_db_connection};

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

  let person = PersonRepository::find_by_user(&mut conn1, user.id)
    .map_err(|e| {
      let params: NotFoundProps = NotFoundProps::new("login".to_string(), user.id, "people".to_string());
      not_found(e.into(), params)
    })?;

  let parents = PersonRepository::find_parents(&mut conn1, user.id)
    .map_err(|e| {
      let params: NotFoundProps = NotFoundProps::new("login".to_string(), user.id, "people".to_string());
      not_found(e.into(), params)
    })?;

  let mut parents_with_roles = Vec::<UserWithRoles>::new();
  for parent in parents.clone() {
    let up: crate::models::users::User = UserRepository::find(&mut conn1, parent.user_id).map_err(|e| {
      let params: NotFoundProps = NotFoundProps::new("login".to_string(), parent.user_id, "user".to_string());
      not_found(e.into(), params)
    })?;

    let mut role_students = Vec::<Role>::new();
    let rp = RoleRepository::find_by_user(&mut conn1, &up)
      .map_err(|e| {
        let params: NotFoundProps = NotFoundProps::new("login".to_string(), parent.user_id, "roles".to_string());
        not_found(e.into(), params)
      })?;

    for r in rp.clone() {
      if r.code == "student" {
        role_students.push(r);
      }
    }

    if role_students.len() > 0 {
      let new_parent_with_role = UserWithRoles::new(up, rp);
      parents_with_roles.push(new_parent_with_role);
    }
  }

  let token = AuthRepository::authorize_user(&user, &credentials)
    .map_err(|e| {
      let message: String = format!("Wrong credentials for service login with username {} with message: {}", credentials.username, e);
      log::error!("{message}");
      Custom(Status::Unauthorized, json!("User or password not valid"))
    })?;

  let signin_data = SignIn::new(
    token.clone(),
    user.clone(),
    person,
    roles,
    parents_with_roles
  );

  cache.set_ex::<_, _, ()>(
    format!("sessions/{}", token),
    user.id,
    3*60*60
  )
  .await
  .map(|_| Custom(Status::Ok, json!(signin_data)))
  .map_err(|e| {
    let params: UnAuthorixedProps = UnAuthorixedProps::new("login".to_string(), &username_cache);
    unauthorized(e.into(), params)
  })
}