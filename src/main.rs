#![feature(proc_macro_hygiene)]
extern crate icns;
mod filesystem;
mod icon_manager;
use github_rs::client::{Executor, Github};
use icon_manager::IconManager;
use once_cell::sync::Lazy;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::io::Error;

#[derive(Deserialize, Debug)]
pub struct Organization {
  login: String,
}

pub const client: Lazy<Github> =
  Lazy::new(|| Github::new(env::var("GITHUB_TOKEN").unwrap());

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  unsafe {
    let c_str = std::ffi::CString::new("/Users/samdenty/Testing/github/Icon")
      .unwrap()
      .as_ptr();
    libc::listxattr(c_str, x, 100, 0);
  }
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

  // let icon_manager = IconManager::new()?;
  // let icon = icon_manager.load("http://google.com/")?;

  // fs::write("icon.rsrc", icon.rsrc)?;
  // fs::write("icon.icns", icon.icns)?;

  // filesystem::mount(icon_manager)?;

  Ok(())
}
