pub mod models;
pub mod schema;

use diesel::prelude::*;
use models::*;
use once_cell::sync::Lazy;
use schema::*;
use std::error::Error;

// const CONNECTION: Lazy<SqliteConnection> = Lazy::new(|| establish_connection());

embed_migrations!("./migrations");

fn establish_connection() -> SqliteConnection {
  let database_url = "file:test.db";

  let conn = SqliteConnection::establish(database_url)
    .unwrap_or_else(|e| panic!("Error connecting to {} {}", database_url, e));
  embedded_migrations::run(&conn).expect("failed to run migrations");

  conn
}

pub fn test() -> Result<(), Box<dyn Error>> {
  let conn = establish_connection();
  // let new_post = NewPost {
  //   title: "hi2",
  //   body: "hi2",
  // };

  // let u = User {
  //   description: Some("hi".into()),
  //   name: "hi".into(),
  // };
  // diesel::insert_into(users::table)
  //   .values(&u)
  //   .execute(&conn)
  //   .expect("Error saving new post");

  // let repo = Repo {
  //   description: Some("hi".into()),
  //   name: "hi".into(),
  // };
  // diesel::insert_into(repos::table)
  //   .values(&repo)
  //   .execute(&conn)
  //   .expect("Error saving new post");

  let users = users::table.load::<User>(&conn)?;
  println!("{:?}", users);
  let user = users.get(0).ok_or("")?;

  // let repos = repos::table.load::<Repo>(&conn)?;

  let repos = Repo::belonging_to(user).load::<Repo>(&conn)?;

  // let b = users::table.find("hi").get_result::<User>(&conn);
  println!("repo {:?}", repos);
  Ok(())
}
