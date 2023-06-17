pub mod students;
pub mod assists;

use diesel::PgConnection;

#[rocket_sync_db_pools::database("postgres")]
pub struct DbConn(PgConnection);