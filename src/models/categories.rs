use chrono::NaiveDateTime;
use diesel::{Queryable, AsChangeset, Insertable};
use serde::{Deserialize, Serialize};

use crate::schema::categories;


#[derive(Queryable, AsChangeset, Deserialize, Serialize)]
#[diesel(table_name=categories)]
pub struct Category {
  #[serde(skip_deserializing)]
  pub id: i32,
  pub category: String,
  pub section: String,
  #[serde(skip_deserializing)]
  pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=categories)]
pub struct  NewCategory {
  pub category: String,
  pub section: String,
}