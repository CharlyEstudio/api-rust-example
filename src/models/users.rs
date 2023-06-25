use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable, Identifiable};
use serde::{Deserialize, Serialize};
use crate::schema::users;

#[derive(Queryable, Debug, Identifiable, Deserialize, Serialize, Clone)]
pub struct User {
  pub id: i32,
  pub username: String,
  pub password: String,
  pub active: Option<bool>,
  pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Clone)]
#[diesel(table_name=users)]
pub struct  NewUser {
  pub username: String,
  pub password: String,
  pub active: Option<bool>,
}