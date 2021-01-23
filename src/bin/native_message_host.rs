use chrome_native_messaging::event_loop;
use open_in_editor::reveal_paths;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::process::Command;
use std::{error::Error, io, path::Path};

#[derive(Serialize, Deserialize)]
struct User {
  user: String,
}

#[derive(Serialize, Deserialize)]
struct Repo {
  user: String,
  repo: String,
}

#[derive(Serialize, Deserialize)]
struct File {
  user: String,
  repo: String,
  tree: String,
  path: String,
}

#[derive(Serialize, Deserialize)]
struct Folder {
  user: String,
  repo: String,
  tree: String,
  path: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum Link {
  User(User),
  Repo(Repo),
  Folder(Folder),
  File(File),
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum FolderLike {
  User(User),
  Repo(Repo),
  Folder(Folder),
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "request")]
enum Message {
  OpenInEditor(Link),
  RevealInFinder(Link),
  OpenInTerminal(FolderLike),
  OpenFile(File),
}

fn main() {
  event_loop(|value: Value| -> Result<Value, Box<dyn Error>> {
    let message = serde_json::from_value(value)?;

    match message {
      Message::OpenInEditor(link) => {
        let path = match &link {
          Link::Repo(link) => format!("{}/{}", link.user, link.repo),
          Link::File(link) => format!("{}/{}/{}", link.user, link.repo, link.path),
          Link::Folder(link) => format!(
            "{}/{}/{}",
            link.user,
            link.repo,
            link.path.clone().unwrap_or("".into())
          ),
          _ => "/".into(),
        };

        Command::new("/usr/local/bin/code-insiders")
          .arg(format!("/Users/samdenty/Projects/repos/test/{}", path))
          .spawn();
      }
      Message::RevealInFinder(link) => {
        let path = match &link {
          Link::User(link) => link.user.clone(),
          Link::Repo(link) => format!("{}/{}", link.user, link.repo),
          Link::File(link) => format!("{}/{}/{}", link.user, link.repo, link.path),
          Link::Folder(link) => format!(
            "{}/{}/{}",
            link.user,
            link.repo,
            link.path.clone().unwrap_or("".into())
          ),
          _ => "/".into(),
        };

        let mut command = Command::new("open");

        if let Link::File(_) = link {
          // reveal file
          command.arg("-R");
        }

        command.arg(format!("/Users/samdenty/Projects/repos/test/{}", path));

        command.spawn();
      }
      Message::OpenInTerminal(link) => {}
      Message::OpenFile(link) => {}
    }

    Ok(json!("ok"))
  });
}
