extern crate basquet;

use rocket_db_pools::Database;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
      .mount("/assists", rocket::routes![
        basquet::routes::assists::get_assists,
        basquet::routes::assists::view_presence,
        basquet::routes::assists::create_assist,
        basquet::routes::assists::update_assist,
        basquet::routes::assists::delete_assist,
      ])
      .mount("/auth", rocket::routes![
        basquet::routes::auth::login,
      ])
      .mount("/categories", rocket::routes![
        basquet::routes::categories::get_categories,
        basquet::routes::categories::view_category,
        basquet::routes::categories::create_category,
        basquet::routes::categories::update_category,
        basquet::routes::categories::delete_category,
      ])
      .mount("/payments", rocket::routes![
        basquet::routes::payments::get_payments,
        basquet::routes::payments::view_payment,
        basquet::routes::payments::create_payment,
        basquet::routes::payments::update_payment,
        basquet::routes::payments::delete_payment,
      ])
      .mount("/people", rocket::routes![
        basquet::routes::people::get_people,
        basquet::routes::people::view_person,
        basquet::routes::people::create_person,
        basquet::routes::people::update_person,
        basquet::routes::people::delete_person,
      ])
      .mount("/students", rocket::routes![
        basquet::routes::students::get_students,
        basquet::routes::students::view_student,
        basquet::routes::students::create_student,
        basquet::routes::students::update_student,
        basquet::routes::students::delete_student,
      ])
      .mount("/users", rocket::routes![
        basquet::routes::users::get_users,
        basquet::routes::users::view_user,
        basquet::routes::users::create_user,
        basquet::routes::users::update_user,
        basquet::routes::users::delete_user,
      ])
      .mount("/type-payments", rocket::routes![
        basquet::routes::type_payments::get_type_payments,
        basquet::routes::type_payments::view_type_payment,
        basquet::routes::type_payments::create_type_payment,
        basquet::routes::type_payments::update_type_payment,
        basquet::routes::type_payments::delete_type_payment,
      ])
      .attach(basquet::routes::DbConn::fairing())
      .attach(basquet::routes::CacheConn::init())
      .launch()
      .await;
}