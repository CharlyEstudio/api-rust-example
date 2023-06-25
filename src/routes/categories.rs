use rocket::{serde::json::{Json, serde_json::json, Value}, response::status::{Custom, NoContent}, http::Status};

use crate::{models::{users::User, props::{ServerErrorProps, NotFoundProps}, categories::{NewCategory, Category}}, functions::responses::{server_error, not_found}, repositories::categories::CategoriesRepository};

use super::DbConn;

#[rocket::get("/")]
pub async fn get_categories(db: DbConn, _user: User) -> Result<Value, Custom<Value>> {
  db.run(|c| {
    CategoriesRepository::find_multiple(c, 100)
      .map(|student| json!(student))
      .map_err(|e| {
        let params: ServerErrorProps = ServerErrorProps::new("get_categories".to_string(), 0, "categories".to_string());
        server_error(e.into(), params)
      })
  }).await
}

#[rocket::get("/<id>")]
pub async fn view_category(id: i32, db: DbConn, _user: User) -> Result<Value, Custom<Value>> {
  db.run(move |c| {
    CategoriesRepository::find(c, id)
      .map(|student| json!(student))
      .map_err(|e| {
        match e {
          diesel::result::Error::NotFound => {
            let params: NotFoundProps = NotFoundProps::new("view_category".to_string(), id, "categories".to_string());
            not_found(e.into(), params)
          },
          _ => {
            let params: ServerErrorProps = ServerErrorProps::new("view_category".to_string(), 0, "categories".to_string());
            server_error(e.into(), params)
          }
        }
      })
  }).await
}

#[rocket::post("/", format="json", data="<new_category>")]
pub async fn create_category(new_category: Json<NewCategory>, db: DbConn, _user: User) -> Result<Custom<Value>, Custom<Value>> {
  db.run(move |c| {
    CategoriesRepository::create(c, new_category.into_inner())
      .map(|category| Custom(Status::Created, json!(category)))
      .map_err(|e| {
        let params: ServerErrorProps = ServerErrorProps::new("create_category".to_string(), 0, "categories".to_string());
        server_error(e.into(), params)
      })
  }).await
}

#[rocket::put("/<id>", format="json", data="<category>")]
pub async fn update_category(id: i32, category: Json<Category>, db: DbConn, _user: User) -> Result<Value, Custom<Value>> {
  db.run(move |c| {
    CategoriesRepository::update(c, id, category.into_inner())
      .map(|category| json!(category))
      .map_err(|e| {
        let params: ServerErrorProps = ServerErrorProps::new("update_category".to_string(), id, "categories".to_string());
        server_error(e.into(), params)
      })
  }).await
}

#[rocket::delete("/<id>")]
pub async fn delete_category(id: i32, db: DbConn, _user: User) -> Result<NoContent, Custom<Value>> {
  db.run(move |c| {
    CategoriesRepository::delete(c, id)
      .map(|_| NoContent)
      .map_err(|e| {
        let params: ServerErrorProps = ServerErrorProps::new("delete_category".to_string(), id, "categories".to_string());
        server_error(e.into(), params)
      })
  }).await
}