use crate::{models::assists::{Assist, NewPresence}, schema::assists};
use diesel::prelude::*;

pub struct AssistsRepository;

impl AssistsRepository {
  pub fn find(c: &mut PgConnection, id: i32) -> QueryResult<Assist> {
    assists::table.find(id).get_result(c)
  }

  pub fn find_multiple(c: &mut PgConnection, limit: i64) -> QueryResult<Vec<Assist>> {
    assists::table.limit(limit).load(c)
  }

  pub fn create(c: &mut PgConnection, new_presence: NewPresence) -> QueryResult<Assist> {
    diesel::insert_into(assists::table)
      .values(new_presence)
      .get_result(c)
  }

  pub fn update(c: &mut PgConnection, id: i32, assist: Assist) -> QueryResult<Assist> {
    diesel::update(assists::table.find(id))
      .set((
        assists::students_id.eq(assist.students_id),
        assists::presence.eq(assist.presence),
      ))
      .get_result(c)
  }
  pub fn delete(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
    diesel::delete(assists::table.find(id)).execute(c)
  }
}