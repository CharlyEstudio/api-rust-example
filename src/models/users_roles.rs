use diesel::{Queryable, Insertable, Associations, Identifiable};
use serde::Serialize;
use crate::models::{users::User, roles::Role};
use crate::schema::users_roles;

#[derive(Queryable, Associations, Identifiable, Debug, Serialize)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Role))]
#[diesel(table_name=users_roles)]
pub struct UserRole {
  pub id: i32,
  pub user_id: i32,
  pub role_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name=users_roles)]
pub struct  NewUserRole {
  pub user_id: i32,
  pub role_id: i32,
}