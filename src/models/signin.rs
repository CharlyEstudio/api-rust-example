use serde::Serialize;

use super::{users::User, people::Person, roles::Role};

#[derive(Serialize)]
pub struct SignIn {
  pub token: String,
  pub status: bool,
  pub user: User,
  pub person: Person,
  pub roles: Vec<Role>,
  pub parents: Vec<UserWithRoles>,
}

impl SignIn {
  pub fn new(token: String, status: bool, user: User, person: Person, roles: Vec<Role>, user_with_roles: Vec<UserWithRoles>) -> Self {
    Self { token, status, user, person, roles, parents: user_with_roles }
  }
}

#[derive(Serialize)]
pub struct UserWithRoles {
  pub user: User,
  pub roles: Vec<Role>
}

impl UserWithRoles {
  pub fn new(user: User, roles: Vec<Role>) -> Self {
    Self { user, roles }
  }
}