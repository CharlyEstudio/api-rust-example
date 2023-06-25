use rocket::{serde::json::{Value, serde_json::json, Json}, response::status::{Custom, NoContent}, http::Status};

use crate::{repositories::assists::AssistsRepository, models::{assists::{NewPresence, Assist}, props::{ServerErrorProps, NotFoundProps}}, routes::DbConn, functions::responses::{server_error, not_found}};

#[rocket::get("/")]
pub async fn get_assists(db: DbConn) -> Result<Value, Custom<Value>> {
  db.run(|c| {
    AssistsRepository::find_multiple(c, 100)
      .map(|presence| json!(presence))
      .map_err(|e| {
        let params: ServerErrorProps = ServerErrorProps::new("get_assists".to_string(), 0, "assists".to_string());
        server_error(e.into(), params)
      })
  }).await
}

#[rocket::get("/<id>")]
pub async fn view_presence(id: i32, db: DbConn) -> Result<Value, Custom<Value>> {
  db.run(move |c| {
    AssistsRepository::find(c, id)
      .map(|presence| json!(presence))
      .map_err(|e| {
        match e {
          diesel::result::Error::NotFound => {
            let params: NotFoundProps = NotFoundProps::new("view_presence".to_string(), id, "assists".to_string());
            not_found(e.into(), params)
          },
          _ => {
            let params: ServerErrorProps = ServerErrorProps::new("view_presence".to_string(), 0, "assists".to_string());
            server_error(e.into(), params)
          }
        }
      })
  }).await
}

#[rocket::post("/", format="json", data="<new_presence>")]
pub async fn create_assist(new_presence: Json<NewPresence>, db: DbConn) -> Result<Custom<Value>, Custom<Value>> {
  db.run(move |c| {
    AssistsRepository::create(c, new_presence.into_inner())
      .map(|presence| Custom(Status::Created, json!(presence)))
      .map_err(|e| {
        let params: ServerErrorProps = ServerErrorProps::new("create_assist".to_string(), 0, "assists".to_string());
        server_error(e.into(), params)
      })
  })
  .await
}

#[rocket::put("/<id>", format="json", data="<presence>")]
pub async fn update_assist(id: i32, presence: Json<Assist>, db: DbConn) -> Result<Value, Custom<Value>> {
  db.run(move |c| {
    AssistsRepository::update(c, id, presence.into_inner())
      .map(|presence| json!(presence))
      .map_err(|e| {
        let params: ServerErrorProps = ServerErrorProps::new("update_assist".to_string(), id, "assists".to_string());
        server_error(e.into(), params)
      })
  }).await
}

#[rocket::delete("/<id>")]
pub async fn delete_assist(id: i32, db: DbConn) -> Result<NoContent, Custom<Value>> {
  db.run(move |c| {
    AssistsRepository::delete(c, id)
      .map(|_| NoContent)
      .map_err(|e| {
        let params: ServerErrorProps = ServerErrorProps::new("delete_assist".to_string(), id, "assists".to_string());
        server_error(e.into(), params)
      })
  }).await
}