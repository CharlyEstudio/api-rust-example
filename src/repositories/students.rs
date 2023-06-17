use crate::{models::students::{Student, NewStudent}, schema::students};
use diesel::prelude::*;

pub struct StudentsRepository;

impl StudentsRepository {
  pub fn find(c: &mut PgConnection, id: i32) -> QueryResult<Student> {
    students::table.find(id).get_result(c)
  }

  pub fn find_multiple(c: &mut PgConnection, limit: i64) -> QueryResult<Vec<Student>> {
    students::table.limit(limit).load(c)
  }

  pub fn create(c: &mut PgConnection, new_student: NewStudent) -> QueryResult<Student> {
    diesel::insert_into(students::table)
      .values(new_student)
      .get_result(c)
  }

  pub fn update(c: &mut PgConnection, id: i32, student: Student) -> QueryResult<Student> {
    diesel::update(students::table.find(id))
      .set((
        students::name.eq(student.name),
        students::email.eq(student.email),
      ))
      .get_result(c)
  }
  pub fn delete(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
    diesel::delete(students::table.find(id)).execute(c)
  }
}