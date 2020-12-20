use super::schema::*;
use diesel::prelude::*;

#[derive(Identifiable, Queryable, Insertable, Debug)]
#[primary_key(name)]
pub struct User {
  pub name: String,
  pub description: Option<String>,
}

#[derive(Identifiable, Queryable, Associations, Insertable, Debug)]
#[belongs_to(User, foreign_key = "user_name")]
pub struct Repo {
  pub id: i32,
  pub user_name: String,
  pub name: String,
  pub description: Option<String>,
  pub private: bool,
  pub fork: bool,
}

// #[derive(Insertable)]
// #[table_name = "users"]
// pub struct NewPost<'a> {
//   pub title: &'a str,
//   pub body: &'a str,
// }
