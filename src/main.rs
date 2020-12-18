#![feature(proc_macro_hygiene)]
#![feature(duration_constants)]
extern crate icns;
mod filesystem;
mod github;
mod icon_manager;

use github::*;
use icon_manager::IconManager;
use serde::Deserialize;
use std::error::Error;
use std::sync::Arc;

#[derive(Deserialize, Debug)]
pub struct Organization {
  login: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  // let client = reqwest::Client::builder()
  //   .proxy(reqwest::Proxy::http("http://localhost:9090")?)
  //   .build()?;

  // let resp = client
  //   .get("https://api.github.com/user/orgs")
  //   .header("User-Agent", "repositories")
  //   .header(
  //     "Authorization",
  //     "token 967d100d7152d2d1cb9e720e5efd06f9c9b836d4",
  //   )
  //   .send()
  //   .await?
  //   .json::<HashMap<String, String>>()
  //   .text()
  //   .await?;

  let mut icon_manager = IconManager::new()?;
  // icon_manager.load("https://example.com")?;

  // database::test().unwrap();
  let user = User::get(Arc::new("samdenty".into()))?;

  println!("{:?}", user);

  filesystem::mount(icon_manager)?;

  Ok(())
}
