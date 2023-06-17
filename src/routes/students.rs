use rocket::{serde::json::{Json, serde_json::json, Value}, response::status::{Custom, NoContent}, http::Status};

use crate::{models::students::{NewStudent, Student}, DbConn, repositories::students::StudentsRepository};

#[rocket::get("/students")]
pub async fn get_students(db: DbConn) -> Result<Value, Custom<Value>> {
  db.run(|c| {
    StudentsRepository::find_multiple(c, 100)
      .map(|student| json!(student))
      .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
  }).await
}

#[rocket::get("/students/<id>")]
pub async fn view_student(id: i32, db: DbConn) -> Result<Value, Custom<Value>> {
  db.run(move |c| {
    StudentsRepository::find(c, id)
      .map(|student| json!(student))
      .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
  }).await
}

#[rocket::post("/students", format="json", data="<new_student>")]
pub async fn create_student(new_student: Json<NewStudent>, db: DbConn) -> Result<Custom<Value>, Custom<Value>> {
  db.run(move |c| {
    StudentsRepository::create(c, new_student.into_inner())
      .map(|student| Custom(Status::Created, json!(student)))
      .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
  }).await
}

#[rocket::put("/students/<id>", format="json", data="<student>")]
pub async fn update_student(id: i32, student: Json<Student>, db: DbConn) -> Result<Value, Custom<Value>> {
  db.run(move |c| {
    StudentsRepository::update(c, id, student.into_inner())
      .map(|student| json!(student))
      .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
  }).await
}

#[rocket::delete("/students/<id>")]
pub async fn delete_student(id: i32, db: DbConn) -> Result<NoContent, Custom<Value>> {
  db.run(move |c| {
    StudentsRepository::delete(c, id)
      .map(|_| NoContent)
      .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
  }).await
}