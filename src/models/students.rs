use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, AsChangeset};
use serde::{Deserialize, Serialize};
use crate::schema::students;

#[derive(Queryable, AsChangeset, Deserialize, Serialize)]
pub struct Student {
  #[serde(skip_deserializing)]
  pub id: i32,
  pub name: String,
  pub email: String,
  #[serde(skip_deserializing)]
  pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=students)]
pub struct  NewStudent {
  pub name: String,
  pub email: String,
}
