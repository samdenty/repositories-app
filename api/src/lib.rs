#![feature(async_closure, map_into_keys_values)]
extern crate cfg_if;
extern crate wasm_bindgen;

pub mod icons;
mod utils;

use cfg_if::cfg_if;
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

#[wasm_bindgen]
pub async fn get_icons(url: String) -> String {
  let icons = icons::get_icons(&url).await.unwrap();
  serde_json::to_string_pretty(&icons).unwrap()
}
