mod repo;
mod user;
pub use repo::*;
pub use user::*;

use github_rs::client::Github;
use once_cell::unsync::Lazy;
use rustbreak::{backend::FileBackend, deser::Yaml, error::RustbreakError, Database, FileDatabase};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, sync::Arc};

pub const CLIENT: Lazy<Github> =
  Lazy::new(|| Github::new(env::var("GITHUB_TOKEN").unwrap()).unwrap());

#[derive(Serialize, Deserialize, Clone)]
struct CurrentUser {
  user: String,
  cache: Vec<User>,
  organizations: Vec<String>,
}
