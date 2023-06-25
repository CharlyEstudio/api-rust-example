use diesel::{PgConnection, Connection};

use crate::{models::users::NewUser, repositories::{users::UserRepository, roles::RoleRepository, auth::AuthRepository}};

fn load_db_connection() -> PgConnection {
  let database_url = std::env::var("DATABASE_URL")
    .expect("Cannot load DB url from env");
  PgConnection::establish(&database_url)
    .expect("Cannot connect to postges")
}

pub fn create_user(username: String, password: String, roles_codes: Vec<String>) {
  let mut c = load_db_connection();

  let password_hash = AuthRepository::hash_password(password).unwrap();

  let new_user = NewUser {username, password: password_hash};
  let user = UserRepository::create(&mut c, new_user, roles_codes).unwrap();
  println!("User created {:?}", user);
  let roles = RoleRepository::find_by_user(&mut c, &user).unwrap();
  println!("Role assigned {:?}", roles);
}

pub fn list_user() {
  let mut c = load_db_connection();

  let users = UserRepository::find_with_roles(&mut c).unwrap();
  for user in users {
    println!("{:?}", user);
  }
}

pub fn delete_user(id: i32) {
  let mut c = load_db_connection();

  UserRepository::delete(&mut c, id).unwrap();
}