use super::User;
use super::CLIENT;
use crate::{database::*, github_api};
use api::icons::get_icons;
use github_rs::client::Executor;
use serde::Deserializer;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};
use std::{collections::HashSet, sync::Arc};

#[derive(Identifiable, Queryable, Insertable, Debug)]
#[primary_key(sha)]
pub struct Blob {
  pub sha: String,
  pub size: i32,
  pub(super) data: Option<Vec<u8>>,
}

impl Blob {
  pub async fn load(owner: &str, repo: &str, sha: &str) -> Result<Blob, Box<dyn Error>> {
    let data = CLIENT
      .get(&github_api!("repos/{}/{}/git/blobs/{}", owner, repo, sha))
      .header("Accept", "application/vnd.github.v3.raw")
      .send()
      .await?
      .bytes()
      .await?;

    let blob = Blob {
      sha: sha.into(),
      size: data.len() as _,
      data: Some(data.to_vec()),
    };

    diesel::replace_into(blobs::table)
      .values(&blob)
      .execute(db())?;

    Ok(blob)
  }

  pub async fn get(owner: &str, repo: &str, blob_sha: &str) -> Result<Blob, Box<dyn Error>> {
    let blob = {
      use self::blobs::dsl::*;
      blobs
        .filter(sha.eq(blob_sha))
        .first::<Blob>(db())
        .optional()?
    };

    match blob {
      Some(blob) => Ok(blob),
      None => Blob::load(owner, repo, blob_sha).await,
    }
  }

  pub async fn get_data(self, owner: &str, repo: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    if let Some(data) = self.data {
      Ok(data)
    } else {
      let blob = Blob::load(owner, repo, &self.sha).await?;
      Ok(blob.data.unwrap())
    }
  }
}
