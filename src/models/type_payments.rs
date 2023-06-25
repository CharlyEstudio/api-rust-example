use chrono::NaiveDateTime;
use diesel::{Queryable, AsChangeset, Insertable};
use serde::{Deserialize, Serialize};

use crate::schema::type_payments;

#[derive(Queryable, AsChangeset, Deserialize, Serialize)]
#[diesel(table_name=type_payments)]
pub struct TypePayment {
  #[serde(skip_deserializing)]
  pub id: i32,
  pub type_payment: String,
  #[serde(skip_deserializing)]
  pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=type_payments)]
pub struct  NewTypePayment {
  pub type_payment: String,
}