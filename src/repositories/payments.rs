use crate::{models::payments::{Payment, NewPayment}, schema::payments};
use diesel::prelude::*;

pub struct PaymentsRepository;

impl PaymentsRepository {
  pub fn find(c: &mut PgConnection, id: i32) -> QueryResult<Payment> {
    payments::table.find(id).get_result(c)
  }

  pub fn find_multiple(c: &mut PgConnection, limit: i64) -> QueryResult<Vec<Payment>> {
    payments::table.limit(limit).load(c)
  }

  pub fn create(c: &mut PgConnection, new_student: NewPayment) -> QueryResult<Payment> {
    diesel::insert_into(payments::table)
      .values(new_student)
      .get_result(c)
  }

  pub fn update(c: &mut PgConnection, id: i32, student: Payment) -> QueryResult<Payment> {
    diesel::update(payments::table.find(id))
      .set((
        payments::student_id.eq(student.student_id),
        payments::amount.eq(student.amount),
        payments::type_payment_id.eq(student.type_payment_id),
        payments::service_id.eq(student.service_id),
      ))
      .get_result(c)
  }

  pub fn delete(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
    diesel::delete(payments::table.find(id)).execute(c)
  }
}