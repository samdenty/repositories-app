use super::repo::Repo;
use crate::database::*;
use github_rs::client::Executor;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::{collections::HashMap, sync::Arc};

#[derive(Identifiable, Queryable, Insertable, AsChangeset, Debug)]
#[primary_key(name)]
pub struct User {
  pub name: String,
  pub description: Option<String>,
}

impl User {
  // pub fn get(login: &str) -> Result<Option<Arc<User>>, Box<dyn Error>> {
  //   if let Some(user) = USERS.read(|entries| entries.get(&login.to_lowercase()).cloned())? {
  //     Ok(Some(user))
  //   } else {
  //     User::load(login)
  //   }
  // }

  // pub fn load_repos(&mut self) -> Result<(), Box<dyn Error>> {
  //   let repos = CLIENT
  //     .get()
  //     .users()
  //     .username(&self.login)
  //     .repos()
  //     .execute::<Vec<RepoResult>>()?
  //     .2
  //     .ok_or("failed to fetch repos")?;

  //   self.repo_ids.clear();
  //   for gh_repo in repos {
  //     self.repo_ids.push(gh_repo.id);
  //     Repo::upsert(gh_repo)?;
  //   }

  //   REPOS.save()?;

  //   return Ok(());
  // }

  // pub fn get_repos() {}

  // pub fn load(login: &str) -> Result<Option<Arc<User>>, Box<dyn Error>> {
  //   let (_, status, gh_user) = CLIENT
  //     .get()
  //     .users()
  //     .username(login)
  //     .execute::<UserResult>()?;

  //   if status == 404 {
  //     return Ok(None);
  //   };

  //   let gh_user = gh_user.ok_or("failed to fetch user")?;
  //   let login = gh_user.login;

  //   let user = Arc::new(User {
  //     login: login.to_string(),
  //     description: gh_user.bio,
  //     repo_ids: Vec::new(),
  //   });

  //   USERS.write(|entries| entries.insert(login.to_lowercase(), user.clone()))?;
  //   USERS.save()?;

  //   Ok(Some(user))
  // }
}

#[derive(Deserialize)]
struct UserResult {
  login: String,
  bio: Option<String>,
}
