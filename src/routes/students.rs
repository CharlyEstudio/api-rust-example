use rocket::{serde::json::{Json, serde_json::json, Value}, response::status::{Custom, NoContent}, http::Status};

use crate::{models::{students::{NewStudent, Student}, props::{ServerErrorProps, NotFoundProps}}, repositories::students::StudentsRepository, routes::DbConn, functions::responses::{server_error, not_found}};


#[rocket::get("/")]
pub async fn get_students(db: DbConn) -> Result<Value, Custom<Value>> {
  db.run(|c| {
    StudentsRepository::find_multiple(c, 100)
      .map(|student| json!(student))
      .map_err(|e| {
        let params: ServerErrorProps = ServerErrorProps::new("get_students".to_string(), 0, "students".to_string());
        server_error(e.into(), params)
      })
  }).await
}

#[rocket::get("/<id>")]
pub async fn view_student(id: i32, db: DbConn) -> Result<Value, Custom<Value>> {
  db.run(move |c| {
    StudentsRepository::find(c, id)
      .map(|student| json!(student))
      .map_err(|e| {
        let params: NotFoundProps = NotFoundProps::new("view_student".to_string(), id, "students".to_string());
        not_found(e.into(), params)
      })
  }).await
}

#[rocket::post("/", format="json", data="<new_student>")]
pub async fn create_student(new_student: Json<NewStudent>, db: DbConn) -> Result<Custom<Value>, Custom<Value>> {
  db.run(move |c| {
    StudentsRepository::create(c, new_student.into_inner())
      .map(|student| Custom(Status::Created, json!(student)))
      .map_err(|e| {
        let params: ServerErrorProps = ServerErrorProps::new("create_student".to_string(), 0, "students".to_string());
        server_error(e.into(), params)
      })
  }).await
}

#[rocket::put("/<id>", format="json", data="<student>")]
pub async fn update_student(id: i32, student: Json<Student>, db: DbConn) -> Result<Value, Custom<Value>> {
  db.run(move |c| {
    StudentsRepository::update(c, id, student.into_inner())
      .map(|student| json!(student))
      .map_err(|e| {
        let params: ServerErrorProps = ServerErrorProps::new("update_student".to_string(), id, "students".to_string());
        server_error(e.into(), params)
      })
  }).await
}

#[rocket::delete("/<id>")]
pub async fn delete_student(id: i32, db: DbConn) -> Result<NoContent, Custom<Value>> {
  db.run(move |c| {
    StudentsRepository::delete(c, id)
      .map(|_| NoContent)
      .map_err(|e| {
        let params: ServerErrorProps = ServerErrorProps::new("delete_student".to_string(), id, "students".to_string());
        server_error(e.into(), params)
      })
  }).await
}