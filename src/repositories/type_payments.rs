use crate::{models::type_payments::{TypePayment, NewTypePayment}, schema::type_payments};
use diesel::prelude::*;

pub struct TypePaymentsRepository;

impl TypePaymentsRepository {
  pub fn find(c: &mut PgConnection, id: i32) -> QueryResult<TypePayment> {
    type_payments::table.find(id).get_result(c)
  }

  pub fn find_multiple(c: &mut PgConnection, limit: i64) -> QueryResult<Vec<TypePayment>> {
    type_payments::table.limit(limit).load(c)
  }

  pub fn create(c: &mut PgConnection, new_student: NewTypePayment) -> QueryResult<TypePayment> {
    diesel::insert_into(type_payments::table)
      .values(new_student)
      .get_result(c)
  }

  pub fn update(c: &mut PgConnection, id: i32, student: TypePayment) -> QueryResult<TypePayment> {
    diesel::update(type_payments::table.find(id))
      .set((
        type_payments::type_payment.eq(student.type_payment),
      ))
      .get_result(c)
  }

  pub fn delete(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
    diesel::delete(type_payments::table.find(id)).execute(c)
  }
}