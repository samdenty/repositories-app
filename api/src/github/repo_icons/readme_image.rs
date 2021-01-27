use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, collections::HashSet};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProjectLink {
  Website,
  Repo,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum KeywordMention {
  Logo,
  Banner,
  RepoName,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ReadmeImage {
  pub src: String,
  /// whether the image was in the primary markdown heading
  pub in_primary_heading: bool,
  /// whether the image was the first/last one in the heading
  pub edge_of_primary_heading: bool,
  /// whether the image mentions a keyword in its src / alt text
  pub keyword_mentions: HashSet<KeywordMention>,
  /// whether the image src points to a file inside of the repo
  pub came_from_repo: bool,
  /// whether the image has links to the projects
  pub link: Option<ProjectLink>,
  /// whether the image has the CSS "align: center"
  pub is_align_center: bool,
  /// whether the image has height or width attributes
  pub has_size_attrs: bool,
}

impl ReadmeImage {
  pub fn weight(&self) -> u8 {
    let mut weight = 0;

    if self.in_primary_heading {
      weight += 2;

      if self.is_align_center {
        weight += 2;
      }

      if self.has_size_attrs {
        weight += 2;
      }

      if self.came_from_repo {
        weight += 4;
      }
    };

    if self.edge_of_primary_heading {
      weight += 4;
    }

    match self.link {
      Some(ProjectLink::Website) => {
        weight += 8;
      }
      Some(ProjectLink::Repo) => {
        weight += 4;
      }
      None => {}
    }

    if self.keyword_mentions.contains(&KeywordMention::Logo) {
      weight += 16
    }

    if self.keyword_mentions.contains(&KeywordMention::Banner) {
      weight += 8
    }

    if self.keyword_mentions.contains(&KeywordMention::RepoName) {
      weight += 4
    }

    weight
  }
}

impl Ord for ReadmeImage {
  fn cmp(&self, other: &Self) -> Ordering {
    other.weight().cmp(&self.weight())
  }
}

impl PartialOrd for ReadmeImage {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}
