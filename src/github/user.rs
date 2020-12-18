use super::{repo::Repo, CLIENT};
use github_rs::client::Executor;
use once_cell::sync::Lazy;
use rustbreak::{backend::FileBackend, deser::Yaml, Database, FileDatabase};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::{collections::HashMap, sync::Arc};

const USERS: Lazy<Database<HashMap<UserLogin, Arc<User>>, FileBackend, Yaml>> =
  Lazy::new(|| FileDatabase::load_from_path_or("/tmp/users", HashMap::new()).unwrap());

pub type UserLogin = Arc<String>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
  pub login: UserLogin,
  pub description: Option<String>,
  pub repos: Vec<u32>,
}

impl User {
  pub fn new(login: UserLogin, description: Option<String>, repos: Vec<u32>) -> Self {
    Self {
      login,
      description,
      repos,
    }
  }

  pub fn get(login: UserLogin) -> Result<Option<Arc<User>>, Box<dyn Error>> {
    if let Some(user) = USERS.read(|entries| entries.get(&login).cloned())? {
      Ok(Some(user))
    } else {
      User::load(login)
    }
  }

  pub fn load(login: UserLogin) -> Result<Option<Arc<User>>, Box<dyn Error>> {
    let (_, status, gh_user) = CLIENT
      .get()
      .users()
      .username(&login)
      .execute::<UserResult>()?;

    if status == 404 {
      return Ok(None);
    };

    let gh_user = gh_user.ok_or("failed to fetch user")?;

    let repos = Repo::load_for_user(login.clone())?;
    let user = Arc::new(User::new(login.clone(), gh_user.bio, repos));

    USERS.write(|e| e.insert(login, user.clone()))?;
    USERS.save()?;

    Ok(Some(user))
  }
}

#[derive(Deserialize)]
struct UserResult {
  bio: Option<String>,
}
