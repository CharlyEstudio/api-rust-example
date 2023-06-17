use diesel::PgConnection;

mod models;
mod repositories;
mod schema;
mod routes;

#[rocket_sync_db_pools::database("postgres")]
pub struct DbConn(PgConnection);

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![
            routes::students::get_students,
            routes::students::view_student,
            routes::students::create_student,
            routes::students::update_student,
            routes::students::delete_student,
        ])
        .attach(DbConn::fairing())
        .launch()
        .await;
}
