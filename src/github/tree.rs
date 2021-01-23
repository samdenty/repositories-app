use super::{Blob, TreeEntry, CLIENT};
use crate::{api::get_icons, database::*, github_api};
use github_rs::client::Executor;
use serde::Deserializer;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};
use std::{collections::HashSet, sync::Arc};

#[derive(Debug)]
pub struct Tree {
  pub sha: String,
  pub entries: Vec<TreeEntry>,
}

impl Tree {
  pub async fn get_dir(
    owner: &str,
    repo: &str,
    tree_sha: &str,
    dir: &str,
  ) -> Result<Tree, Box<dyn Error>> {
    Tree::cache(owner, repo, tree_sha).await?;

    let mut re = "^".to_string();
    re += &regex::escape(dir);
    if !dir.is_empty() {
      re += r"/";
    }
    re += r"[^/]*$";

    let entries = {
      use self::trees::dsl::*;
      trees
        .filter(sha.eq(tree_sha).and(functions::regexp(re, path)))
        .load::<TreeEntry>(db())?
    };

    Ok(Tree {
      sha: tree_sha.into(),
      entries,
    })
  }

  pub fn is_cached(tree_sha: &str) -> Result<bool, Box<dyn Error>> {
    use self::trees::dsl::*;
    use diesel::dsl::*;

    Ok(select(exists(trees.filter(sha.eq(tree_sha)))).get_result::<bool>(db())?)
  }

  pub async fn cache(owner: &str, repo: &str, tree_sha: &str) -> Result<(), Box<dyn Error>> {
    if !Tree::is_cached(tree_sha)? {
      Tree::load_all(owner, repo, tree_sha).await?;
    }

    Ok(())
  }

  pub async fn get_all(owner: &str, repo: &str, tree_sha: &str) -> Result<Tree, Box<dyn Error>> {
    let entries = {
      use self::trees::dsl::*;
      trees.filter(sha.eq(tree_sha)).load::<TreeEntry>(db())?
    };

    if entries.is_empty() {
      Ok(Tree::load_all(owner, repo, tree_sha).await?)
    } else {
      let sha = entries.first().unwrap().sha.clone();
      Ok(Tree { entries, sha })
    }
  }

  pub async fn load_all(owner: &str, repo: &str, sha: &str) -> Result<Tree, Box<dyn Error>> {
    let res = CLIENT
      .get(&github_api!(
        "repos/{}/{}/git/trees/{}?recursive=1",
        owner,
        repo,
        sha
      ))
      .send()
      .await?;

    let data = res.json::<RootData>().await?;

    let mut entries = Vec::new();

    for tree in data.tree {
      let blob_sha = {
        if tree.kind == "blob" {
          let blob = Blob {
            sha: tree.sha.clone(),
            size: tree.size.unwrap(),
            data: None,
          };

          diesel::insert_into(blobs::table)
            .values(&blob)
            .on_conflict_do_nothing()
            .execute(db())?;

          Some(tree.sha)
        } else {
          None
        }
      };

      let tree_entry = TreeEntry {
        sha: data.sha.clone(),
        path: tree.path,
        mode: tree.mode,
        blob_sha,
      };
      entries.push(tree_entry);
    }

    diesel::insert_into(trees::table)
      .values(&entries)
      .execute(db())?;

    Ok(Tree {
      sha: data.sha,
      entries,
    })
  }
}

#[derive(Deserialize)]
pub struct RootData {
  pub sha: String,
  pub tree: Vec<TreeData>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeData {
  pub path: String,
  pub mode: String,
  #[serde(rename = "type")]
  pub kind: String,
  pub sha: String,
  pub size: Option<i32>,
}
