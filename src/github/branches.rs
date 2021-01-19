use super::{Blob, Repo, Tree, CLIENT};
use super::{TreeEntry, User};
use crate::{database::*, github_api};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};
use std::{collections::HashSet, sync::Arc};

#[derive(Identifiable, Queryable, Insertable, Debug, Clone)]
#[primary_key(owner, repo, name)]
#[table_name = "repo_branches"]
pub struct Branch {
  owner: String,
  repo: String,
  pub name: String,
  tree_sha: String,
}

impl Branch {
  pub async fn get_dir(&self, dir: &str) -> Result<Tree, Box<dyn Error>> {
    Tree::get_dir(&self.owner, &self.repo, &self.tree_sha, dir).await
  }

  pub async fn get_entry(&self, path: &str) -> Result<TreeEntry, Box<dyn Error>> {
    TreeEntry::get(&self.owner, &self.repo, &self.tree_sha, path).await
  }

  pub async fn get_default(owner_name: &str, repo_name: &str) -> Result<Branch, Box<dyn Error>> {
    let branch_name = {
      use self::repos::dsl::*;
      repos
        .filter(owner.like(owner_name).and(name.like(repo_name)))
        .select(default_branch)
        .first::<String>(&*DB)
        .optional()?
    };
    let branch_name = match branch_name {
      Some(branch) => branch,
      None => Repo::load(owner_name, repo_name).await?.default_branch,
    };

    let branch = {
      use self::repo_branches::dsl::*;
      repo_branches
        .filter(owner.like(owner_name).and(repo.like(repo_name)))
        .first::<Branch>(&*DB)
        .optional()?
    };
    let branch = match branch {
      Some(branch) => branch,
      None => Branches::load(owner_name, repo_name)
        .await?
        .get_branch(&branch_name)
        .unwrap(),
    };

    Ok(branch)
  }
}

#[derive(Debug)]
pub struct Branches {
  pub entries: Vec<Branch>,
}

impl Branches {
  pub async fn get(owner_name: &str, repo_name: &str) -> Result<Branches, Box<dyn Error>> {
    let entries = {
      use self::repo_branches::dsl::*;
      repo_branches
        .filter(owner.like(owner_name).and(repo.like(repo_name)))
        .load::<Branch>(&*DB)?
    };

    if entries.is_empty() {
      let branches = Branches::load(owner_name, repo_name).await?;
      Ok(branches)
    } else {
      let branches = Branches { entries };
      Ok(branches)
    }
  }

  pub async fn load(owner_name: &str, repo_name: &str) -> Result<Branches, Box<dyn Error>> {
    let res = CLIENT
      .get(&github_api!("repos/{}/{}/branches", owner_name, repo_name))
      .send()
      .await?;

    let data = res.json::<Vec<BranchData>>().await?;

    let mut entries = Vec::new();
    for ref_data in data {
      let _ref = Branch {
        owner: owner_name.into(),
        repo: repo_name.into(),
        name: ref_data.name,
        tree_sha: ref_data.commit.sha,
      };

      entries.push(_ref);
    }

    {
      use self::repo_branches::dsl::*;

      diesel::delete(repo_branches.filter(owner.like(owner_name).and(repo.like(repo_name))))
        .execute(&*DB)?;

      diesel::insert_into(repo_branches)
        .values(&entries)
        .execute(&*DB)?;
    };

    let branches = Branches { entries };
    Ok(branches)
  }

  pub fn get_branch(&self, name: &str) -> Option<Branch> {
    self.entries.iter().find(|r| r.name == name).cloned()
  }
}

#[derive(Deserialize)]
pub struct BranchData {
  pub name: String,
  pub commit: CommitData,
}

#[derive(Deserialize)]
pub struct CommitData {
  pub sha: String,
}
