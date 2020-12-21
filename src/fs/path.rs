use std::path::Path;

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
  Repo(String, String),
  Content(String, String, String),
}

pub fn parse(path: &Path) -> Kind {
  let path: Vec<_> = path
    .iter()
    .map(|p| p.to_str().unwrap().to_string())
    .collect();

  let user = || path.get(1).unwrap().into();
  let repo = || path.get(2).unwrap().into();

  if let Some(last) = path.last() {
    if last == "Icon\r" {
      match path.len() {
        3 => return Kind::Icon(Icon::User(user())),
        4 => return Kind::Icon(Icon::Repo(user(), repo())),

        _ => {}
      }
    }
  }

  match path.len() {
    1 => Kind::Root,
    2 => Kind::User(user()),
    3 => Kind::Repo(user(), repo()),
    _ => {
      let content_path: Vec<String> = path[3..].iter().map(|p| p.into()).collect();
      Kind::Content(user(), repo(), content_path.join("/"))
    }
  }
}
