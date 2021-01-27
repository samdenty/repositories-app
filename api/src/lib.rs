#![feature(async_closure, map_into_keys_values)]
#[macro_use]
extern crate cfg_if;
extern crate wasm_bindgen;
#[macro_use]
extern crate log;
#[macro_use]
extern crate cached;
extern crate regex;

pub mod github;
pub mod icons;
mod macros;
mod utils;

use cfg_if::cfg_if;
use log::Level;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen(start)]
pub fn main() {
  console_log::init_with_level(Level::Info);
  utils::set_panic_hook();
}

#[wasm_bindgen]
pub async fn get_icons(url: String) -> String {
  let icons = icons::get_icons(&url).await.unwrap();
  serde_json::to_string_pretty(&icons).unwrap()
}

#[wasm_bindgen]
pub fn set_token(token: String) {
  github::set_token(&token);
}

#[wasm_bindgen]
pub async fn get_repo_icons(owner: String, repo: String) -> String {
  let images = github::RepoIcons::new(&owner, &repo, None)
    .await
    .unwrap()
    .get_images()
    .await;

  serde_json::to_string_pretty(&images).unwrap()
}
