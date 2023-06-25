use crate::{models::people::{Person, NewPerson}, schema::people};
use diesel::prelude::*;

pub struct PersonRepository;

impl PersonRepository {
  pub fn find(c: &mut PgConnection, id: i32) -> QueryResult<Person> {
    people::table.find(id).get_result(c)
  }

  pub fn find_multiple(c: &mut PgConnection, limit: i64) -> QueryResult<Vec<Person>> {
    people::table.limit(limit).load(c)
  }

  pub fn create(c: &mut PgConnection, new_student: NewPerson) -> QueryResult<Person> {
    diesel::insert_into(people::table)
      .values(new_student)
      .get_result(c)
  }

  pub fn update(c: &mut PgConnection, id: i32, student: Person) -> QueryResult<Person> {
    diesel::update(people::table.find(id))
      .set((
        people::name.eq(student.name),
        people::first_name.eq(student.first_name),
        people::surname.eq(student.surname),
        people::user_id.eq(student.user_id),
        people::parent_id.eq(student.parent_id),
      ))
      .get_result(c)
  }

  pub fn delete(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
    diesel::delete(people::table.find(id)).execute(c)
  }
}