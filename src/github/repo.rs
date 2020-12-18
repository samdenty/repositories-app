use super::{user::UserLogin, CLIENT};
use github_rs::client::Executor;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use rustbreak::{backend::FileBackend, deser::Yaml, Database, FileDatabase};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};
use std::{collections::HashSet, sync::Arc};

const REPOS: Lazy<Database<HashMap<u32, RepoEntry>, FileBackend, Yaml>> =
  Lazy::new(|| FileDatabase::load_from_path_or("/tmp/repos", HashMap::new()).unwrap());

pub type RepoEntry = Arc<RwLock<Repo>>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Repo {
  pub name: String,
  pub tags: HashSet<String>,
  pub description: Option<String>,
  pub is_private: bool,
  pub is_fork: bool,
}

impl Repo {
  pub fn load_for_user(login: UserLogin) -> Result<Vec<u32>, Box<dyn Error>> {
    let repos = CLIENT
      .get()
      .users()
      .username(&login)
      .repos()
      .execute::<Vec<RepoResult>>()?
      .2
      .ok_or("failed to fetch repos")?;

    let mut repo_ids = Vec::new();
    for gh_repo in repos {
      repo_ids.push(gh_repo.id);
      Repo::upsert(gh_repo)?;
    }

    REPOS.save()?;

    return Ok(repo_ids);
  }

  pub fn get(id: u32) -> Result<Option<RepoEntry>, Box<dyn Error>> {
    if let Some(repo) = REPOS.read(|entries| entries.get(&id).cloned())? {
      Ok(Some(repo))
    } else {
      Repo::load(id)
    }
  }

  pub fn load(id: u32) -> Result<Option<RepoEntry>, Box<dyn Error>> {
    let (_, status, gh_repo) = CLIENT
      .get()
      .custom_endpoint(&format!("/repositories/{}", id))
      .execute::<RepoResult>()?;

    if status == 404 {
      return Ok(None);
    };

    let gh_repo = gh_repo.ok_or("failed to fetch repo")?;
    let repo = Repo::upsert(gh_repo).map(|repo| Some(repo));

    REPOS.save()?;

    repo
  }

  fn upsert(gh_repo: RepoResult) -> Result<RepoEntry, Box<dyn Error>> {
    if let Some(repo) = REPOS.read(|entries| entries.get(&gh_repo.id).cloned())? {
      {
        let mut repo = repo.write();
        repo.name = gh_repo.name;
        repo.description = gh_repo.description;
        repo.is_private = gh_repo.private;
        repo.is_fork = gh_repo.fork;
      }
      return Ok(repo);
    }

    let id = gh_repo.id;
    let repo = Arc::new(RwLock::new(Repo {
      name: gh_repo.name,
      tags: HashSet::new(),
      description: gh_repo.description,
      is_private: gh_repo.private,
      is_fork: gh_repo.fork,
    }));

    REPOS.write(|e| e.insert(id, repo.clone()))?;
    Ok(repo)
  }
}

#[derive(Deserialize)]
struct RepoResult {
  id: u32,
  name: String,
  private: bool,
  fork: bool,
  description: Option<String>,
  created_at: String,
  pushed_at: String,
}
