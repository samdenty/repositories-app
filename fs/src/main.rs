#![feature(
  proc_macro_hygiene,
  duration_constants,
  async_closure,
  impl_trait_in_bindings
)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
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
mod macros;

use github::Repo;
use github::Tree;
use icon_manager::IconManager;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct Organization {
  login: String,
}

// #[tokio::main]
fn main() -> Result<(), Box<dyn Error>> {
  env_logger::init();
  color_backtrace::install();

  let now = std::time::Instant::now();
  // let a = Repo::get("alainm23", "planner")
  //   .await?
  //   .ok_or("")?
  //   .get_icons()
  //   .await?;

  // let a = Tree::get(
  //   "samdenty",
  //   "console-feed",
  //   "9c9469cfe8b26e321fc4f3b6669c22ad1360e193",
  // )
  // .await?;

  // println!("{:?} {:.2?}", a, now.elapsed());

  std::process::Command::new("umount")
    .arg("./test")
    .spawn()
    .expect("failed to unmount");

  let icon_manager = IconManager::new()?;
  fs::mount(icon_manager)?;

  // let browser = headless_chrome::Browser::default().unwrap();
  // let tab = browser.new_tab().unwrap();
  // drop(browser);

  // tab.navigate_to("https://google.com").unwrap();

  // let a = icon_manager
  //     .load_repo("http://127.0.0.1:8081/a.html")
  //     .unwrap();
  // std::fs::write("./test.icns", a.icns.clone())?;

  Ok(())
}
