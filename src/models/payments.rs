use chrono::NaiveDateTime;
use diesel::{Queryable, AsChangeset, Insertable};
use serde::{Deserialize, Serialize};

use crate::schema::payments;

#[derive(Queryable, AsChangeset, Deserialize, Serialize)]
#[diesel(table_name=payments)]
pub struct Payment {
  #[serde(skip_deserializing)]
  pub id: i32,
  pub student_id: i32,
  pub amount: Option<f64>,
  pub type_payment_id: i32,
  pub service_id: Option<i32>,
  #[serde(skip_deserializing)]
  pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=payments)]
pub struct  NewPayment {
  pub student_id: i32,
  pub amount: Option<f64>,
  pub type_payment_id: i32,
  pub service_id: Option<i32>,
}