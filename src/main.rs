mod models;
mod repositories;
mod schema;
mod routes;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/students", rocket::routes![
            routes::students::get_students,
            routes::students::view_student,
            routes::students::create_student,
            routes::students::update_student,
            routes::students::delete_student,
        ])
        .mount("/assists", rocket::routes![
            routes::assists::get_assists,
            routes::assists::view_presence,
            routes::assists::create_assist,
            routes::assists::update_assist,
            routes::assists::delete_assist,
        ])
        .attach(crate::routes::DbConn::fairing())
        .launch()
        .await;
}
