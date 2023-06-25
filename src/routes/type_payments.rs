use rocket::{serde::json::{Json, serde_json::json, Value}, response::status::{Custom, NoContent}, http::Status};

use crate::{models::{users::User, props::{ServerErrorProps, NotFoundProps}, type_payments::{NewTypePayment, TypePayment}}, functions::responses::{server_error, not_found}, repositories::type_payments::TypePaymentsRepository};

use super::DbConn;

#[rocket::get("/")]
pub async fn get_type_payments(db: DbConn, _user: User) -> Result<Value, Custom<Value>> {
  db.run(|c| {
    TypePaymentsRepository::find_multiple(c, 100)
      .map(|student| json!(student))
      .map_err(|e| {
        let params: ServerErrorProps = ServerErrorProps::new("get_type_payments".to_string(), 0, "type_payments".to_string());
        server_error(e.into(), params)
      })
  }).await
}

#[rocket::get("/<id>")]
pub async fn view_type_payment(id: i32, db: DbConn, _user: User) -> Result<Value, Custom<Value>> {
  db.run(move |c| {
    TypePaymentsRepository::find(c, id)
      .map(|student| json!(student))
      .map_err(|e| {
        match e {
          diesel::result::Error::NotFound => {
            let params: NotFoundProps = NotFoundProps::new("view_type_payment".to_string(), id, "type_payments".to_string());
            not_found(e.into(), params)
          },
          _ => {
            let params: ServerErrorProps = ServerErrorProps::new("view_type_payment".to_string(), 0, "type_payments".to_string());
            server_error(e.into(), params)
          }
        }
      })
  }).await
}

#[rocket::post("/", format="json", data="<new_type_payment>")]
pub async fn create_type_payment(new_type_payment: Json<NewTypePayment>, db: DbConn, _user: User) -> Result<Custom<Value>, Custom<Value>> {
  db.run(move |c| {
    TypePaymentsRepository::create(c, new_type_payment.into_inner())
      .map(|tp| Custom(Status::Created, json!(tp)))
      .map_err(|e| {
        let params: ServerErrorProps = ServerErrorProps::new("create_type_payment".to_string(), 0, "type_payments".to_string());
        server_error(e.into(), params)
      })
  }).await
}

#[rocket::put("/<id>", format="json", data="<type_payment>")]
pub async fn update_type_payment(id: i32, type_payment: Json<TypePayment>, db: DbConn, _user: User) -> Result<Value, Custom<Value>> {
  db.run(move |c| {
    TypePaymentsRepository::update(c, id, type_payment.into_inner())
      .map(|tp| json!(tp))
      .map_err(|e| {
        let params: ServerErrorProps = ServerErrorProps::new("update_type_payment".to_string(), id, "type_payments".to_string());
        server_error(e.into(), params)
      })
  }).await
}

#[rocket::delete("/<id>")]
pub async fn delete_type_payment(id: i32, db: DbConn, _user: User) -> Result<NoContent, Custom<Value>> {
  db.run(move |c| {
    TypePaymentsRepository::delete(c, id)
      .map(|_| NoContent)
      .map_err(|e| {
        let params: ServerErrorProps = ServerErrorProps::new("delete_type_payment".to_string(), id, "type_payments".to_string());
        server_error(e.into(), params)
      })
  }).await
}