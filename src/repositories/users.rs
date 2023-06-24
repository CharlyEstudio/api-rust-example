use diesel::{PgConnection, QueryResult};
use diesel::prelude::*;

use crate::models::users_roles::{NewUserRole, UserRole};
use crate::models::roles::NewRole;
use crate::schema::users_roles;
use crate::{models::users::{User, NewUser}, schema::users};

use super::roles::RoleRepository;

pub struct UserRepository;

impl UserRepository {
  pub fn _find(c: &mut PgConnection, id: i32) -> QueryResult<User> {
    users::table.find(id).get_result(c)
  }

  pub fn _find_multiple(c: &mut PgConnection, limit: i64) -> QueryResult<Vec<User>> {
    users::table.limit(limit).load(c)
  }

  pub fn create(c: &mut PgConnection, new_user: NewUser, roles_codes: Vec<String>) -> QueryResult<User> {
    let user = diesel::insert_into(users::table)
      .values(new_user)
      .get_result::<User>(c)?;

    for role_code in roles_codes {
      let new_user_role = {
        if let Ok(role) = RoleRepository::find_by_code(c, role_code.to_owned()) {
          NewUserRole {user_id: user.id, role_id: role.id}
        } else {
          let new_role = NewRole {name: role_code.to_owned(), code: role_code.to_owned()};
          let role = RoleRepository::create(c, new_role)?;
          NewUserRole {user_id: user.id, role_id: role.id}
        }
      };

      diesel::insert_into(users_roles::table)
          .values(new_user_role)
          .get_result::<UserRole>(c)?;
    }

    Ok(user)
  }

  pub fn _update(c: &mut PgConnection, id: i32, user: User) -> QueryResult<User> {
    diesel::update(users::table.find(id))
      .set((
        users::username.eq(user.username),
        users::password.eq(user.password),
      ))
      .get_result(c)
  }

  pub fn _delete(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
    diesel::delete(users::table.find(id)).execute(c)
  }
}