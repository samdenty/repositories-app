#![feature(proc_macro_hygiene)]
#![feature(duration_constants)]
extern crate icns;
mod filesystem;
mod icon_manager;
use github_rs::client::{Executor, Github};
use icon_manager::IconManager;
use once_cell::sync::Lazy;
use serde::Deserialize;
use serde_json::Value;
use std::convert::TryInto;
use std::fs;
use std::io::Error;
use std::{collections::HashMap, env};

#[derive(Deserialize, Debug)]
pub struct Organization {
  login: String,
}

pub const client: Lazy<Github> =
  Lazy::new(|| Github::new(env::var("GITHUB_TOKEN").unwrap()).unwrap());

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  unsafe {
    // let c_path = std::ffi::CString::new("/").unwrap();
    // let c_name = std::ffi::CString::new("com.apple.FinderInfoa").unwrap();

    // let desired_size = libc::getxattr(
    //   c_path.as_ptr(),
    //   c_name.as_ptr() as *const libc::c_char,
    //   std::ptr::null_mut(),
    //   0,
    //   0,
    //   0, // options
    // );

    // if desired_size == -1 {
    //   println!("no {:?} {}", c_path, std::io::Error::last_os_error());
    // };

    // let mut buf: Vec<u8> = vec![0; desired_size as usize];

    // libc::getxattr(
    //   c_path.as_ptr(),
    //   c_name.as_ptr() as *const libc::c_char,
    //   buf.as_mut_ptr() as *mut libc::c_void,
    //   desired_size.try_into().unwrap(),
    //   0,
    //   0, // options
    // );

    // fs::write("./b", buf);

    // let s = String::from_utf8_lossy(&buf);
    // println!("buf: {:x?}", buf);
    // println!(
    //   "buf: {:x?}",
    //   "com.apple.FinderInfo\u{0}com.apple.ResourceFork\u{0}".as_bytes()
    // );

    // let c_path = std::ffi::CString::new("/").unwrap();

    // let err_or_size = libc::listxattr(c_path.as_ptr(), std::ptr::null_mut(), 0, 0);

    // if err_or_size == -1 {
    //   println!("no {:?} {}", c_path, std::io::Error::last_os_error());
    // };

    // let mut buf: Vec<u8> = vec![0; err_or_size as usize];

    // libc::listxattr(c_path.as_ptr(), buf.as_mut_ptr() as *mut i8, buf.len(), 0);

    // let s = String::from_utf8_lossy(&buf);
    // println!("buf: {:x?}", s);
  }
  // println!(
  //   "buf: {:x?}",
  //   "com.apple.FinderInfo\u{0}com.apple.ResourceFork\u{0}".as_bytes()
  // );
  // }
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

  filesystem::mount(icon_manager)?;
  // let icon = icon_manager.load("http://google.com/")?;

  // fs::write("icon.rsrc", icon.rsrc)?;
  // fs::write("icon.icns", icon.icns)?;

  Ok(())
}
