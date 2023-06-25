use crate::{models::categories::{Category, NewCategory}, schema::categories};
use diesel::prelude::*;

pub struct CategoriesRepository;

impl CategoriesRepository {
  pub fn find(c: &mut PgConnection, id: i32) -> QueryResult<Category> {
    categories::table.find(id).get_result(c)
  }

  pub fn find_multiple(c: &mut PgConnection, limit: i64) -> QueryResult<Vec<Category>> {
    categories::table.limit(limit).load(c)
  }

  pub fn create(c: &mut PgConnection, new_catogory: NewCategory) -> QueryResult<Category> {
    diesel::insert_into(categories::table)
      .values(new_catogory)
      .get_result(c)
  }

  pub fn update(c: &mut PgConnection, id: i32, category: Category) -> QueryResult<Category> {
    diesel::update(categories::table.find(id))
      .set((
        categories::category.eq(category.category),
        categories::section.eq(category.section),
      ))
      .get_result(c)
  }

  pub fn delete(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
    diesel::delete(categories::table.find(id)).execute(c)
  }
}