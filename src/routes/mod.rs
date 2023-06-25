pub mod auth;
pub mod users;
pub mod students;
pub mod assists;

use diesel::PgConnection;
use rocket_db_pools::{deadpool_redis, Database};

#[rocket_sync_db_pools::database("postgres")]
pub struct DbConn(PgConnection);

#[derive(Database)]
#[database("redis")]
pub struct CacheConn(deadpool_redis::Pool);