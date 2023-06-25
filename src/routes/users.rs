use rocket::{serde::json::{Json, serde_json::json, Value}, response::status::{Custom, NoContent}, http::Status};

use crate::{repositories::{users::UserRepository, auth::AuthRepository}, models::{users::{NewUser, User}, props::ServerErrorProps}, functions::responses::server_error};

use super::DbConn;

#[rocket::get("/")]
pub async fn get_users(db: DbConn) -> Result<Value, Custom<Value>> {
  db.run(|c| {
    UserRepository::find_multiple(c, 10)
    .map(|users| json!(users))
    .map_err(|e| {
      let params: ServerErrorProps = ServerErrorProps::new("get_users".to_string(), 0, "users".to_string());
      server_error(e.into(), params)
    })
  }).await
}

#[rocket::get("/<id>")]
pub async fn view_user(id: i32, db: DbConn) -> Result<Value, Custom<Value>> {
  db.run(move |c| {
    UserRepository::find(c, id)
    .map(|users| json!(users))
    .map_err(|e| {
      let params: ServerErrorProps = ServerErrorProps::new("view_user".to_string(), 0, "users".to_string());
      server_error(e.into(), params)
    })
  }).await
}

#[rocket::post("/", format="json", data="<new_user>")]
pub async fn create_user(new_user: Json<NewUser>, db: DbConn) -> Result<Custom<Value>, Custom<Value>> {
  db.run(move |c| {
    let password = new_user.clone().into_inner().password;
    let username = new_user.clone().into_inner().username;
    let password_hash = AuthRepository::hash_password(password).unwrap();
    let new_user_hash = NewUser {username: username.clone(), password: password_hash};
    let role_code = vec!["student".to_string()];

    UserRepository::create(c, new_user_hash, role_code)
      .map(|user| Custom(Status::Created, json!(user)))
      .map_err(|e| {
        let params: ServerErrorProps = ServerErrorProps::new("create_user".to_string(), 0, username);
        server_error(e.into(), params)
      })
  }).await
}

#[rocket::put("/<id>", format="json", data="<user>")]
pub async fn update_user(id: i32, user: Json<User>, db: DbConn) -> Result<Value, Custom<Value>> {
  db.run(move |c| {
    let password = user.clone().into_inner().password;
    let username = user.clone().into_inner().username;
    let password_hash = AuthRepository::hash_password(password).unwrap();
    let update_user_hash = User {id: user.id, username, password: password_hash, created_at: user.created_at};

    UserRepository::update(c, id, update_user_hash)
    .map(|users| json!(users))
    .map_err(|e| {
      let params: ServerErrorProps = ServerErrorProps::new("get_users".to_string(), 0, "users".to_string());
      server_error(e.into(), params)
    })
  }).await
}

#[rocket::delete("/<id>")]
pub async fn delete_user(id: i32, db: DbConn) -> Result<NoContent, Custom<Value>> {
  db.run(move |c| {
    UserRepository::delete(c, id)
      .map(|_| NoContent)
      .map_err(|e| {
        let params: ServerErrorProps = ServerErrorProps::new("delete_user".to_string(), id, "users".to_string());
        server_error(e.into(), params)
      })
  }).await
}