use crate::regex;
use once_cell::sync::Lazy;
use regex::Regex;
use std::{error::Error, path::Path};

#[derive(Debug)]
pub enum Icon {
  User(String),
  Repo(String, String),
}

#[derive(Debug)]
pub enum Kind {
  Root,
  Icon(Icon),
  User(String),
  DefaultTree(String, String, String),
  CustomTree(String, String, String),
}

pub fn parse(path: &Path) -> Result<Kind, Box<dyn Error>> {
  let path: Vec<_> = path.iter().map(|p| p.to_str().unwrap()).collect();
  let is_icon = path.last() == Some(&"Icon\r");

  if path.len() == 1 {
    return Ok(Kind::Root);
  }

  let owner: String = path[1].into();
  if !regex!(r"^([a-z\d]+-)*[a-z\d]+$").is_match(&owner) {
    return Err("invalid username".into());
  };

  if path.len() == 2 {
    return Ok(Kind::User(owner));
  }

  let repo = path[2].into();
  if !regex!(r"^([A-Za-z0-9_.-]){0,100}$").is_match(&owner) {
    return Err("invalid repo name".into());
  };

  if is_icon {
    match path.len() {
      3 => return Ok(Kind::Icon(Icon::User(owner))),
      4 => return Ok(Kind::Icon(Icon::Repo(owner, repo))),
      _ => {}
    }
  }

  let path = &path[3..];

  match path.get(0).cloned() {
    Some("^tree") => {
      let path = &path[0..];
      Ok(Kind::CustomTree(owner, repo, path.join("/")))
    }
    _ => Ok(Kind::DefaultTree(owner, repo, path.join("/"))),
  }
}
