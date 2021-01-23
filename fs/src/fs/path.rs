use crate::regex;
use once_cell::sync::Lazy;
use regex::Regex;
use std::{error::Error, path::Path};

#[derive(Debug)]
pub enum Icon<'a> {
  User(&'a str),
  Repo(&'a str, &'a str),
}

#[derive(Debug)]
pub enum Kind<'a> {
  Root,
  Icon(Icon<'a>),
  User(&'a str),
  DefaultTree(&'a str, &'a str, String),
  CustomTree(&'a str, &'a str, String),
}

pub fn parse(path: &Path) -> Result<Kind, Box<dyn Error>> {
  let path: Vec<_> = path.iter().map(|p| p.to_str().unwrap()).collect();
  let is_icon = path.last() == Some(&"Icon\r");

  if path.len() == 1 {
    return Ok(Kind::Root);
  }

  let owner = path[1];
  if !regex!(r"^([a-z\d]+-)*[a-z\d]+$").is_match(owner) {
    return Err("invalid username".into());
  };

  if path.len() == 2 {
    return Ok(Kind::User(owner));
  }

  let repo = path[2];
  if !regex!(r"^([A-Za-z0-9_.-]){0,100}$").is_match(owner) {
    return Err("invalid repo name".into());
  };

  if [".git", "node_modules"].contains(&repo) {
    return Err("reserved name".into());
  }

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
