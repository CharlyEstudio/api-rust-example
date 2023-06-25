use rocket::{serde::json::{Json, serde_json::json, Value}, response::status::{Custom, NoContent}, http::Status};

use crate::{models::{users::User, props::{ServerErrorProps, NotFoundProps}, people::{NewPerson, Person}}, functions::responses::{server_error, not_found}, repositories::people::PersonRepository};

use super::DbConn;

#[rocket::get("/")]
pub async fn get_people(db: DbConn, _user: User) -> Result<Value, Custom<Value>> {
  db.run(|c| {
    PersonRepository::find_multiple(c, 100)
      .map(|student| json!(student))
      .map_err(|e| {
        let params: ServerErrorProps = ServerErrorProps::new("get_people".to_string(), 0, "people".to_string());
        server_error(e.into(), params)
      })
  }).await
}

#[rocket::get("/<id>")]
pub async fn view_person(id: i32, db: DbConn, _user: User) -> Result<Value, Custom<Value>> {
  db.run(move |c| {
    PersonRepository::find(c, id)
      .map(|student| json!(student))
      .map_err(|e| {
        match e {
          diesel::result::Error::NotFound => {
            let params: NotFoundProps = NotFoundProps::new("view_person".to_string(), id, "people".to_string());
            not_found(e.into(), params)
          },
          _ => {
            let params: ServerErrorProps = ServerErrorProps::new("view_person".to_string(), 0, "people".to_string());
            server_error(e.into(), params)
          }
        }
      })
  }).await
}

#[rocket::post("/", format="json", data="<new_person>")]
pub async fn create_person(new_person: Json<NewPerson>, db: DbConn, _user: User) -> Result<Custom<Value>, Custom<Value>> {
  db.run(move |c| {
    PersonRepository::create(c, new_person.into_inner())
      .map(|student| Custom(Status::Created, json!(student)))
      .map_err(|e| {
        let params: ServerErrorProps = ServerErrorProps::new("create_person".to_string(), 0, "people".to_string());
        server_error(e.into(), params)
      })
  }).await
}

#[rocket::put("/<id>", format="json", data="<person>")]
pub async fn update_person(id: i32, person: Json<Person>, db: DbConn, _user: User) -> Result<Value, Custom<Value>> {
  db.run(move |c| {
    PersonRepository::update(c, id, person.into_inner())
      .map(|person| json!(person))
      .map_err(|e| {
        let params: ServerErrorProps = ServerErrorProps::new("update_person".to_string(), id, "people".to_string());
        server_error(e.into(), params)
      })
  }).await
}

#[rocket::delete("/<id>")]
pub async fn delete_person(id: i32, db: DbConn, _user: User) -> Result<NoContent, Custom<Value>> {
  db.run(move |c| {
    PersonRepository::delete(c, id)
      .map(|_| NoContent)
      .map_err(|e| {
        let params: ServerErrorProps = ServerErrorProps::new("delete_person".to_string(), id, "people".to_string());
        server_error(e.into(), params)
      })
  }).await
}