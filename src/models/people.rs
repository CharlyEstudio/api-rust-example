use chrono::NaiveDateTime;
use diesel::{Queryable, AsChangeset, Insertable};
use serde::{Deserialize, Serialize};

use crate::schema::people;


#[derive(Queryable, AsChangeset, Deserialize, Serialize)]
#[diesel(table_name=people)]
pub struct Person {
  #[serde(skip_deserializing)]
  pub id: i32,
  pub name: String,
  pub first_name: String,
  pub surname: Option<String>,
  pub user_id: i32,
  pub parent_id: i32,
  #[serde(skip_deserializing)]
  pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=people)]
pub struct  NewPerson {
  pub name: String,
  pub first_name: String,
  pub surname: String,
  pub user_id: i32,
  pub parent_id: i32,
}