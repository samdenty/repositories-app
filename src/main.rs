#![feature(
    proc_macro_hygiene,
    duration_constants,
    async_closure,
    impl_trait_in_bindings
)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate hex_literal;
#[macro_use]
extern crate diesel_migrations;

mod database;
mod fs;
mod github;
mod icon_manager;

use icon_manager::IconManager;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct Organization {
    login: String,
}

// #[tokio::main]
fn main() -> Result<(), Box<dyn Error>> {
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

    // database::test().unwrap();

    std::process::Command::new("umount")
        .arg("./test")
        .spawn()
        .expect("failed to unmount");

    let mut icon_manager = IconManager::new()?;
    fs::mount(icon_manager)?;

    // icon_manager.load("https://example.com")?;

    Ok(())
}
