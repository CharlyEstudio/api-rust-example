use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, AsChangeset};
use serde::{Deserialize, Serialize};
use crate::schema::assists;

#[derive(Queryable, AsChangeset, Deserialize, Serialize)]
pub struct Assist {
  #[serde(skip_deserializing)]
  pub id: i32,
  pub students_id: i32,
  pub presence: Option<NaiveDateTime>,
  #[serde(skip_deserializing)]
  pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=assists)]
pub struct NewPresence {
  pub students_id: i32,
  pub presence: Option<NaiveDateTime>,
}