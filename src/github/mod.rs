mod repo;
mod user;
pub use repo::Repo;
pub use user::User;

use github_rs::client::Github;
use once_cell::sync::Lazy;
use rustbreak::{deser::Yaml, error::RustbreakError, FileDatabase};
use serde::{Deserialize, Serialize};
use std::env;

pub const CLIENT: Lazy<Github> =
  Lazy::new(|| Github::new(env::var("GITHUB_TOKEN").unwrap()).unwrap());

#[derive(Serialize, Deserialize, Clone)]
struct CurrentUser {
  user: String,
  cache: Vec<User>,
  organizations: Vec<String>,
}

pub fn test() -> Result<(), RustbreakError> {
  let db = FileDatabase::<CurrentUser, Yaml>::load_from_path_or(
    "/tmp/aaa",
    CurrentUser {
      cache: Vec::new(),
      user: "samdenty".into(),
      // user: User::new("samdenty", Some("hello")),
      organizations: Vec::new(),
    },
  )?;

  db.save()?;

  Ok(())
}
