mod blob;
mod branches;
mod repo;
mod tree;
mod tree_entry;
mod user;

pub use blob::*;
pub use branches::*;
pub use repo::*;
pub use tree::*;
pub use tree_entry::*;
pub use user::*;

use once_cell::unsync::Lazy;
use reqwest::{header, Client};
use std::env;

pub const CLIENT: Lazy<Client> = Lazy::new(|| {
  let mut headers = header::HeaderMap::new();

  headers.insert(
    header::AUTHORIZATION,
    header::HeaderValue::from_str(&format!("Bearer {}", env::var("GITHUB_TOKEN").unwrap()))
      .unwrap(),
  );

  let client = reqwest::Client::builder()
    .user_agent("repositories")
    .default_headers(headers)
    .build()
    .unwrap();

  client
});
