extern crate basquet;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
      .mount("/students", rocket::routes![
        basquet::routes::students::get_students,
        basquet::routes::students::view_student,
        basquet::routes::students::create_student,
        basquet::routes::students::update_student,
        basquet::routes::students::delete_student,
      ])
      .mount("/assists", rocket::routes![
        basquet::routes::assists::get_assists,
        basquet::routes::assists::view_presence,
        basquet::routes::assists::create_assist,
        basquet::routes::assists::update_assist,
        basquet::routes::assists::delete_assist,
      ])
      .attach(basquet::routes::DbConn::fairing())
      .launch()
      .await;
}