use rocket::{request::{FromRequest, Outcome}, Request, http::Status, fairing::{Fairing, Info, Kind}, Response};
use rocket_db_pools::{Connection, deadpool_redis::redis::AsyncCommands};

use crate::{models::users::User, routes::{CacheConn, DbConn}, repositories::users::UserRepository};

#[rocket::options("/<_route_args..>")]
pub fn options(_route_args: Option<std::path::PathBuf>) {
  // Just to add CORS header via the fairing.
}

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
  fn info(&self) -> Info {
    Info { name: "Append CORS headers in responses", kind: Kind::Response }
  }

  async fn on_response<'r>(&self, _req: &'r Request<'_>, res: &mut Response<'r>) {
    res.set_raw_header("Access-Control-Allow-Origin", "*");
    res.set_raw_header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE");
    res.set_raw_header("Access-Control-Allow-Headers", "*");
    res.set_raw_header("Access-Control-Allow-Credentials", "true");
  }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
  type Error = ();
  async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    let session_header = req.headers().get_one("Authorization")
      .map(|v| {v.split_whitespace().collect::<Vec<_>>()})
      .filter(|v| v.len() == 2 && v[0] == "Bearer");

    if let Some(session_value) = session_header {
      let mut cache = req
        .guard::<Connection<CacheConn>>()
        .await
        .expect("Cannot connect to redis in request guard");

      let db = req
        .guard::<DbConn>()
        .await
        .expect("Cannot connect to db in request guard");

      let result = cache.get::<_, i32>(format!("sessions/{}", session_value[1])).await;
      if let Ok(user_id) = result {
        return match db.run(move |c| UserRepository::find(c, user_id)).await {
          Ok(user) => Outcome::Success(user),
          _ => Outcome::Failure((Status::Unauthorized, ())),
        }
      }
    }

    Outcome::Failure((Status::Unauthorized, ()))
  }
}