use rocket::{serde::json::{Json, serde_json::json, Value}, response::status::{Custom, NoContent}, http::Status};

use crate::{models::{users::User, props::{ServerErrorProps, NotFoundProps}, payments::{NewPayment, Payment}}, functions::responses::{server_error, not_found}, repositories::payments::PaymentsRepository};

use super::DbConn;

#[rocket::get("/")]
pub async fn get_payments(db: DbConn, _user: User) -> Result<Value, Custom<Value>> {
  db.run(|c| {
    PaymentsRepository::find_multiple(c, 100)
      .map(|student| json!(student))
      .map_err(|e| {
        let params: ServerErrorProps = ServerErrorProps::new("get_payments".to_string(), 0, "payments".to_string());
        server_error(e.into(), params)
      })
  }).await
}

#[rocket::get("/<id>")]
pub async fn view_payment(id: i32, db: DbConn, _user: User) -> Result<Value, Custom<Value>> {
  db.run(move |c| {
    PaymentsRepository::find(c, id)
      .map(|student| json!(student))
      .map_err(|e| {
        match e {
          diesel::result::Error::NotFound => {
            let params: NotFoundProps = NotFoundProps::new("view_payment".to_string(), id, "payments".to_string());
            not_found(e.into(), params)
          },
          _ => {
            let params: ServerErrorProps = ServerErrorProps::new("view_person".to_string(), 0, "payments".to_string());
            server_error(e.into(), params)
          }
        }
      })
  }).await
}

#[rocket::post("/", format="json", data="<new_payment>")]
pub async fn create_payment(new_payment: Json<NewPayment>, db: DbConn, _user: User) -> Result<Custom<Value>, Custom<Value>> {
  db.run(move |c| {
    PaymentsRepository::create(c, new_payment.into_inner())
      .map(|student| Custom(Status::Created, json!(student)))
      .map_err(|e| {
        let params: ServerErrorProps = ServerErrorProps::new("create_payment".to_string(), 0, "payments".to_string());
        server_error(e.into(), params)
      })
  }).await
}

#[rocket::put("/<id>", format="json", data="<payment>")]
pub async fn update_payment(id: i32, payment: Json<Payment>, db: DbConn, _user: User) -> Result<Value, Custom<Value>> {
  db.run(move |c| {
    PaymentsRepository::update(c, id, payment.into_inner())
      .map(|payment| json!(payment))
      .map_err(|e| {
        let params: ServerErrorProps = ServerErrorProps::new("update_payment".to_string(), id, "payments".to_string());
        server_error(e.into(), params)
      })
  }).await
}

#[rocket::delete("/<id>")]
pub async fn delete_payment(id: i32, db: DbConn, _user: User) -> Result<NoContent, Custom<Value>> {
  db.run(move |c| {
    PaymentsRepository::delete(c, id)
      .map(|_| NoContent)
      .map_err(|e| {
        let params: ServerErrorProps = ServerErrorProps::new("delete_payment".to_string(), id, "payments".to_string());
        server_error(e.into(), params)
      })
  }).await
}