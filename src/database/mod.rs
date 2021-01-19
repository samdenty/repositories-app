mod schema;
pub use diesel::prelude::*;
pub use schema::*;

use crate::github::*;
use lru::LruCache;
use once_cell::sync::Lazy;
use regex::Regex;
use std::{
  error::Error,
  sync::{Arc, Mutex},
};

pub const DB: Lazy<SqliteConnection> = Lazy::new(|| establish_connection());

embed_migrations!("./migrations");

pub mod functions {
  use diesel::sql_types::*;

  sql_function!(fn regexp(regex: Text, text: Text) -> Bool);
}

fn establish_connection() -> SqliteConnection {
  let database_url = "file:test.db";

  let conn = SqliteConnection::establish(database_url)
    .unwrap_or_else(|e| panic!("Error connecting to {} {}", database_url, e));

  {
    let cache: Arc<Mutex<LruCache<String, Arc<Regex>>>> = Arc::new(Mutex::new(LruCache::new(16)));

    functions::regexp::register_impl(&conn, move |regex: String, text: String| {
      let mut cache = cache.lock().unwrap();
      let re = cache.get(&regex).cloned().unwrap_or_else(|| {
        let re = Arc::new(Regex::new(&regex).unwrap());
        cache.put(regex, re.clone());
        re
      });

      re.is_match(&text)
    })
    .unwrap();
  }

  embedded_migrations::run(&conn).expect("failed to run migrations");

  conn
}

pub fn test() -> Result<(), Box<dyn Error>> {
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

  let a = User {
    name: "samdenty".into(),
    description: None,
  };
  let b: User = a.save_changes(&*DB)?;

  let users = users::table.load::<User>(&*DB)?;
  println!("{:?}", users);
  let user = users.get(0).ok_or("")?;

  // let repos = repos::table.load::<Repo>(&conn)?;

  let repos = Repo::belonging_to(user).load::<Repo>(&*DB)?;

  // let b = users::table.find("hi").get_result::<User>(&conn);
  println!("repo {:?}", repos);
  Ok(())
}
