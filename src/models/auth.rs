use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Credentials {
  pub username: String,
  pub password: String,
}