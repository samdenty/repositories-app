use std::{path::Path, sync::Arc};

#[derive(Debug)]
pub enum PathKind {
  Root,
  User(String),
  Repo {
    user: String,
    repo: String,
  },
  Content {
    user: String,
    repo: String,
    path: String,
  },
}

pub fn parse_path(path: &Path) -> PathKind {
  let path: Vec<_> = path
    .iter()
    .map(|p| p.to_str().unwrap().to_string())
    .collect();

  let user = path.get(1).map(|user| user.into());
  let repo = path.get(2).map(|user| user.into());

  match path.len() {
    1 => PathKind::Root,
    2 => PathKind::User(user.unwrap()),
    3 => PathKind::Repo {
      user: user.unwrap(),
      repo: repo.unwrap(),
    },
    _ => {
      let content_path: Vec<String> = path[3..].iter().map(|p| p.into()).collect();
      PathKind::Content {
        user: user.unwrap(),
        repo: repo.unwrap(),
        path: content_path.join("/"),
      }
    }
  }
}
