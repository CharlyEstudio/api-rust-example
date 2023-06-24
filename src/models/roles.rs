use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable, Identifiable};
use crate::schema::roles;

#[derive(Queryable, Identifiable, Debug)]
pub struct Role {
  pub id: i32,
  pub code: String,
  pub name: String,
  pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name=roles)]
pub struct  NewRole {
  pub code: String,
  pub name: String,
}