use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable, Identifiable};
use crate::schema::users;

#[derive(Queryable, Debug, Identifiable)]
pub struct User {
  pub id: i32,
  pub username: String,
  pub password: String,
  pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name=users)]
pub struct  NewUser {
  pub username: String,
  pub password: String,
}