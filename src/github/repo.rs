use super::User;
use super::CLIENT;
use crate::database::*;
use github_rs::client::Executor;
use parking_lot::RwLock;
use serde::Deserializer;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};
use std::{collections::HashSet, sync::Arc};

#[derive(Identifiable, Queryable, Associations, Insertable, Debug)]
#[belongs_to(User, foreign_key = "user_name")]
pub struct Repo {
  pub id: i32,
  pub user_name: String,
  pub name: String,
  pub description: Option<String>,
  pub private: bool,
  pub fork: bool,
}

impl Repo {
  pub fn load_for_user(login: &str) -> Result<Vec<Repo>, Box<dyn Error>> {
    use self::repos::dsl::*;

    // let a = || -> Result<(), Box<dyn Error>> {
    //   let (_, status, repo) = CLIENT
    //     .get()
    //     .custom_endpoint(&format!("repositories/{}", id))
    //     .execute::<Vec<NewRepo>>()?;
    //   let all_repos = CLIENT
    //     .get()
    //     .users()
    //     .username(login)
    //     .repos()
    //     .execute::<Vec<NewRepo>>()?;

    //   Ok(())
    // };

    let all_repos = CLIENT
      .get()
      .users()
      .username(login)
      .repos()
      .execute::<Vec<NewRepo>>()?
      .2
      .ok_or("failed to fetch repos")?;

    for repo in &all_repos {
      diesel::insert_into(repos)
        .values(repo)
        .on_conflict(id)
        .do_update()
        .set(repo)
        .execute(&*DB)?;
    }

    if all_repos.len() == 0 {
      Ok(Vec::new())
    } else {
      Repo::get_for_user(login)
    }
  }

  pub fn get_for_user(login: &str) -> Result<Vec<Repo>, Box<dyn Error>> {
    use self::repos::dsl::*;
    let all_repos = repos.filter(user_name.like(login)).load::<Repo>(&*DB)?;

    if all_repos.len() == 0 {
      Repo::load_for_user(&login)
    } else {
      Ok(all_repos)
    }
  }

  // pub fn get(id: u32) -> Result<Option<RepoEntry>, Box<dyn Error>> {
  //   if let Some(repo) = REPOS.read(|entries| entries.get(&id).cloned())? {
  //     Ok(Some(repo))
  //   } else {
  //     Repo::load(id)
  //   }
  // }

  pub fn load(id: u32) -> Result<Option<NewRepo>, Box<dyn Error>> {
    let (_, status, repo) = CLIENT
      .get()
      .custom_endpoint(&format!("repositories/{}", id))
      .execute::<NewRepo>()?;

    if status == 404 {
      return Ok(None);
    };

    let repo = repo.ok_or("failed to fetch repo")?;

    diesel::insert_into(repos::table)
      .values(&repo)
      .execute(&*DB)?;

    Ok(Some(repo))
  }
}

#[derive(Insertable, AsChangeset, Debug)]
#[table_name = "repos"]
pub struct NewRepo {
  id: i32,
  user_name: String,
  name: String,
  description: Option<String>,
  private: bool,
  fork: bool,
}

impl<'de> Deserialize<'de> for NewRepo {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    #[derive(Deserialize)]
    struct RepoOwner {
      login: String,
    }

    #[derive(Deserialize)]
    struct Repo {
      id: i32,
      name: String,
      owner: RepoOwner,
      private: bool,
      fork: bool,
      description: Option<String>,
    }

    let repo = Repo::deserialize(deserializer)?;

    Ok(NewRepo {
      id: repo.id,
      user_name: repo.owner.login,
      name: repo.name,
      description: repo.description,
      private: repo.private,
      fork: repo.fork,
    })
  }
}
