use super::{Blob, Tree, CLIENT};
use crate::{api::get_icons, database::*, github_api};
use fuse::FileType;
use github_rs::client::Executor;
use serde::Deserializer;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};
use std::{collections::HashSet, sync::Arc};

#[derive(Identifiable, Queryable, Insertable, Debug)]
#[table_name = "trees"]
#[primary_key(sha, path)]
pub struct TreeEntry {
  pub sha: String,
  pub path: String,
  pub mode: String,
  pub(super) blob_sha: Option<String>,
}

impl TreeEntry {
  pub async fn get(
    owner: &str,
    repo: &str,
    tree_sha: &str,
    entry_path: &str,
  ) -> Result<TreeEntry, Box<dyn Error>> {
    Tree::cache(owner, repo, tree_sha).await?;

    let entry = {
      use self::trees::dsl::*;
      trees
        .filter(sha.eq(tree_sha).and(path.eq(entry_path)))
        .first::<TreeEntry>(db())?
    };

    Ok(entry)
  }

  pub async fn blob(&self, owner: &str, repo: &str) -> Result<Option<Blob>, Box<dyn Error>> {
    Ok(match &self.blob_sha {
      Some(sha) => Some(Blob::get(owner, repo, &sha).await?),
      None => None,
    })
  }

  pub fn is_file(&self) -> bool {
    self.blob_sha.is_some()
  }
}

impl Into<FileType> for &TreeEntry {
  fn into(self) -> FileType {
    if self.is_file() {
      FileType::RegularFile
    } else {
      FileType::Directory
    }
  }
}
