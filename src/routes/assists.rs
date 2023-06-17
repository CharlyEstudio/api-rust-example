use rocket::{serde::json::{Value, serde_json::json, Json}, response::status::{Custom, NoContent}, http::Status};

use crate::{DbConn, repositories::assists::AssistsRepository, models::assists::{NewPresence, Assist}};

#[rocket::get("/assists")]
pub async fn get_assists(db: DbConn) -> Result<Value, Custom<Value>> {
  db.run(|c| {
    AssistsRepository::find_multiple(c, 100)
      .map(|presence| json!(presence))
      .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
  }).await
}

#[rocket::get("/assists/<id>")]
pub async fn view_presence(id: i32, db: DbConn) -> Result<Value, Custom<Value>> {
  db.run(move |c| {
    AssistsRepository::find(c, id)
      .map(|presence| json!(presence))
      .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
  }).await
}

#[rocket::post("/assists", format="json", data="<new_presence>")]
pub async fn create_assist(new_presence: Json<NewPresence>, db: DbConn) -> Result<Custom<Value>, Custom<Value>> {
  db.run(move |c| {
    AssistsRepository::create(c, new_presence.into_inner())
      .map(|presence| Custom(Status::Created, json!(presence)))
      .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
  })
  .await
}

#[rocket::put("/assists/<id>", format="json", data="<presence>")]
pub async fn update_assist(id: i32, presence: Json<Assist>, db: DbConn) -> Result<Value, Custom<Value>> {
  db.run(move |c| {
    AssistsRepository::update(c, id, presence.into_inner())
      .map(|presence| json!(presence))
      .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
  }).await
}

#[rocket::delete("/assists/<id>")]
pub async fn delete_assist(id: i32, db: DbConn) -> Result<NoContent, Custom<Value>> {
  db.run(move |c| {
    AssistsRepository::delete(c, id)
      .map(|_| NoContent)
      .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
  }).await
}