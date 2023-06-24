pub mod students;
pub mod assists;

use std::error::Error;

use diesel::PgConnection;
use rocket::{response::status::Custom, http::Status, serde::json::{serde_json::json, Value}};

use crate::models::props::ServerErrorProps;

#[rocket_sync_db_pools::database("postgres")]
pub struct DbConn(PgConnection);

pub fn server_error(e: Box<dyn Error>, params: ServerErrorProps) -> Custom<Value> {
  let error = format!("Service {} at ID {} into table {} with error: {}", params.service, params.id, params.table, e);
  log::error!("{error}");
  Custom(Status::InternalServerError, json!(error))
}