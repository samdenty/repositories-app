mod repo;
mod user;
pub use repo::*;
pub use user::*;

use github_rs::client::Github;
use once_cell::unsync::Lazy;
use reqwest::{header, Client};
use std::env;

pub const CLIENT: Lazy<Github> =
  Lazy::new(|| Github::new(env::var("GITHUB_TOKEN").unwrap()).unwrap());

// pub const CLIENT2: Lazy<Client> = Lazy::new(|| {
//   let mut headers = header::HeaderMap::new();

//   headers.insert(
//     header::AUTHORIZATION,
//     header::HeaderValue::from_static(&format!("Bearer {}", env::var("GITHUB_TOKEN").unwrap())),
//   );

//   let client = reqwest::Client::builder()
//     .user_agent("repositories")
//     .default_headers(headers)
//     .build()
//     .unwrap();

//   client
// });
