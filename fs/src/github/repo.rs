use super::{Branch, Branches, Tree, User, CLIENT};
use crate::{database::*, github_api};
use api::icons::{get_icons, Icon};
use github_rs::client::Executor;
use serde::Deserializer;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};
use std::{collections::HashSet, sync::Arc};

pub struct UserRepos {}

impl UserRepos {
  pub async fn load(owner_name: &str) -> Result<Vec<RepoData>, Box<dyn Error>> {
    use self::repos::dsl::*;
    let all_repos = CLIENT
      .get(&github_api!("users/{}/repos", owner_name))
      .send()
      .await?
      .json::<Vec<RepoData>>()
      .await?;

    for repo in &all_repos {
      diesel::insert_into(repos)
        .values(repo)
        .on_conflict((owner, name))
        .do_update()
        .set(repo)
        .execute(db())?;
    }

    Ok(all_repos)
  }

  pub async fn get(owner_name: &str) -> Result<Vec<Repo>, Box<dyn Error>> {
    let get_repos = || {
      use self::repos::dsl::*;
      repos.filter(owner.like(owner_name)).load::<Repo>(db())
    };

    let mut all_repos = get_repos()?;
    if all_repos.is_empty() {
      UserRepos::load(&owner_name).await?;
      all_repos = get_repos()?;
    }

    Ok(all_repos)
  }
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[belongs_to(User, foreign_key = "owner")]
#[primary_key(owner, name)]
pub struct Repo {
  pub owner: String,
  pub name: String,
  default_branch: String,
  pub description: Option<String>,
  pub homepage: Option<String>,
  pub private: bool,
  pub fork: bool,
}

impl Repo {
  pub async fn get(owner_name: &str, repo_name: &str) -> Result<Repo, Box<dyn Error>> {
    let get_repo = || {
      use self::repos::dsl::*;
      repos
        .filter(owner.like(owner_name).and(name.like(repo_name)))
        .first::<Repo>(db())
    };

    match get_repo() {
      Ok(repo) => Ok(repo),
      Err(_) => {
        Repo::load(owner_name, repo_name).await?;
        Ok(get_repo()?)
      }
    }
  }

  pub async fn load(owner: &str, name: &str) -> Result<RepoData, Box<dyn Error>> {
    let res = CLIENT
      .get(&github_api!("repos/{}/{}", owner, name))
      .send()
      .await?;

    if res.status() == 404 {
      return Err("could not find repo".into());
    };

    let repo = res.json::<RepoData>().await?;

    diesel::insert_into(repos::table)
      .values(&repo)
      .execute(db())?;

    Ok(repo)
  }

  pub async fn get_icons(&self) -> Result<Vec<Icon>, Box<dyn Error>> {
    if let Some(ref url) = self.homepage {
      let urls = get_icons(url).await?;
      Ok(urls)
    } else {
      Ok(Vec::new())
    }
  }

  pub async fn default_branch(&self) -> Result<Branch, Box<dyn Error>> {
    let branches = Branches::get(&self.owner, &self.name).await?;
    branches
      .get_branch(&self.default_branch)
      .ok_or("missing default branch".into())
  }
}

#[derive(Insertable, AsChangeset, Debug)]
#[table_name = "repos"]
pub struct RepoData {
  pub owner: String,
  pub name: String,
  pub description: Option<String>,
  pub default_branch: String,
  pub homepage: Option<String>,
  pub private: bool,
  pub fork: bool,
}

impl<'de> Deserialize<'de> for RepoData {
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
      name: String,
      owner: RepoOwner,
      private: bool,
      default_branch: String,
      fork: bool,
      description: Option<String>,
      #[serde(with = "serde_with::rust::string_empty_as_none")]
      homepage: Option<String>,
    }

    let repo = Repo::deserialize(deserializer)?;

    Ok(RepoData {
      owner: repo.owner.login,
      name: repo.name,
      default_branch: repo.default_branch,
      description: repo.description,
      private: repo.private,
      fork: repo.fork,
      homepage: repo.homepage,
    })
  }
}
