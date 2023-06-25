use std::error::Error;

use rocket::{response::status::Custom, http::Status, serde::json::{serde_json::json, Value}};

use crate::models::props::{ServerErrorProps, NotFoundProps, NotFoundUsernameProps, UnAuthorixedProps};

pub fn server_error(e: Box<dyn Error>, params: ServerErrorProps) -> Custom<Value> {
  let error = format!("Service {} at ID {} into table {} with error: {}", params.service, params.id, params.table, e);
  log::error!("{error}");
  Custom(Status::InternalServerError, json!(error))
}

pub fn not_found(e: Box<dyn Error>, params: NotFoundProps) -> Custom<Value> {
  let message: String = format!("Not found at service {} with ID {} into table {} with message: {}", params.service, params.id, params.table, e);
  log::info!("{message}");
  Custom(Status::NotFound, json!(message))
}

pub fn not_found_user(e: Box<dyn Error>, params: NotFoundUsernameProps) -> Custom<Value> {
  let message: String = format!("Not found user at service {} with username {} into table {} with message: {}", params.service, params.username, params.table, e);
  log::info!("{message}");
  Custom(Status::NotFound, json!(message))
}

pub fn unauthorized(e: Box<dyn Error>, params: UnAuthorixedProps) -> Custom<Value> {
  let message: String = format!("User not autorized for service {} with username {} with message: {}", params.service, params.username, e);
  log::warn!("{message}");
  Custom(Status::Unauthorized, json!("User or password not valid"))
}