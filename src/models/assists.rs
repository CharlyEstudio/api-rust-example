use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, AsChangeset};
use crate::schema::assists;

#[derive(Queryable, AsChangeset)]
pub struct Assist {
  pub id: i32,
  pub students_id: i32,
  pub presence: Option<NaiveDateTime>,
  pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name=assists)]
pub struct NewPresence {
  pub students_id: i32,
  pub presence: Option<NaiveDateTime>,
}