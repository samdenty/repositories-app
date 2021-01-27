mod badges;
mod primary_heading;
mod readme_image;
mod repo_redirect;

use self::{
  primary_heading::PrimaryHeading,
  readme_image::{KeywordMention, ProjectLink, ReadmeImage},
  repo_redirect::is_same_repo,
};
use crate::{github_api_get, regex};
use badges::is_badge;
use regex::{escape, Regex, RegexBuilder};
use reqwest::Url;
use scraper::{node::Element, ElementRef, Html, Selector};
use std::{collections::HashSet, error::Error};

pub struct RepoIcons {
  owner: String,
  repo: String,
  homepage: Option<Url>,
  document: Html,

  href_base: Url,
  src_base: Url,
}

impl RepoIcons {
  pub async fn new(
    owner: &str,
    repo: &str,
    homepage: Option<Url>,
  ) -> Result<RepoIcons, Box<dyn Error>> {
    let owner = owner.to_lowercase();
    let repo = repo.to_lowercase();

    let res = github_api_get!("repos/{}/{}/readme", owner, repo)
      .header("Accept", "application/vnd.github.html")
      .send()
      .await?;

    if !res.status().is_success() {
      return Err("failed to fetch".into());
    }

    let body = res.text().await?;

    let document = Html::parse_document(&body);

    let href_base = Url::parse(&format!(
      "https://github.com/{}/{}/blob/master/",
      owner, repo
    ))?;

    let src_base = Url::parse(&format!(
      "https://github.com/{}/{}/raw/master/",
      owner, repo
    ))?;

    Ok(Self {
      owner,
      repo,
      homepage,
      document,
      href_base,
      src_base,
    })
  }

  pub async fn get_images(&self) -> Vec<ReadmeImage> {
    let mut primary_heading = PrimaryHeading::new(&self.document);

    let mut images = Vec::new();
    for element_ref in self.document.select(&Selector::parse("img[src]").unwrap()) {
      let elem = element_ref.value();

      let src = elem
        .attr("data-canonical-src")
        .or(elem.attr("src"))
        .and_then(|src| self.src_base.join(src).ok())
        .unwrap();
      let cdn_src = elem
        .attr("data-canonical-src")
        .and(elem.attr("src"))
        .and_then(|src| self.src_base.join(src).ok());

      info!("is_badge {:?}", is_badge(&src));
      if is_badge(&src) == Some(true) {
        continue;
      }

      let mut is_align_center = false;
      let mut link = None;
      for element_ref in element_ref.ancestors().map(ElementRef::wrap).flatten() {
        let element = element_ref.value();

        if element.attr("align") == Some("center") {
          is_align_center = true;
        }

        if element.name() == "a" && link.is_none() {
          link = match element
            .attr("href")
            .and_then(|href| self.href_base.join(href).ok())
          {
            Some(href) => self.get_project_link(href, &src).await,
            None => None,
          };
        }
      }

      let in_primary_heading = primary_heading.contains(element_ref);

      if src.path().contains("PANDA_sitting") {
        info!("{:#?}", element_ref.id());
      }

      let keyword_mentions = self.find_keyword_mentions(&src, elem.attr("alt"));
      let came_from_repo = self.comes_from_repo(&src).await.unwrap_or(false);
      let has_size_attrs = elem.attr("width").or(elem.attr("height")).is_some();

      images.push(ReadmeImage {
        src: cdn_src.unwrap_or(src).to_string(),
        in_primary_heading,
        edge_of_primary_heading: false,
        keyword_mentions,
        came_from_repo,
        link,
        is_align_center,
        has_size_attrs,
      })
    }

    let mut iter = images.iter_mut().enumerate().peekable();
    while let Some((idx, image)) = iter.next() {
      if image.in_primary_heading
        && (idx == 0
          || iter
            .peek()
            .map(|(_, image)| !image.in_primary_heading)
            .unwrap_or(true))
      {
        image.edge_of_primary_heading = true;
      };
    }

    images.sort();

    warn!(
      "{:#?}",
      images
        .iter()
        .map(|img| (img.src.clone(), img.weight()))
        .collect::<Vec<_>>()
    );

    images
  }

  async fn get_project_link(&self, link_href: Url, img_src: &Url) -> Option<ProjectLink> {
    // if the img points to the same url as the link
    // then its a default url generated by github
    let mut img_blob_url = img_src.clone();
    img_blob_url.set_path(&img_src.path().replacen("/raw/", "/blob/", 1));
    if link_href == img_blob_url {
      return None;
    };

    let domain = link_href.domain()?.to_lowercase();

    // check for github pages
    if let Some(res) = regex!(r"^([^.])+\.github\.(com|io)$").captures(&domain) {
      let user = &res[1];

      // USERNAME.github.io
      if self.repo == domain {
        return Some(ProjectLink::Website);
      }

      if let Some(res) = regex!("^/([^/]+)").captures(link_href.path()) {
        let repo = &res[1];
        if self.is_same_repo_as(user, repo).await {
          return Some(ProjectLink::Website);
        }
      }
    }

    if self
      .homepage
      .as_ref()
      .and_then(|u| u.domain().map(|d| d.to_lowercase()))
      .map(|d| domain == d)
      .unwrap_or(false)
    {
      return Some(ProjectLink::Website);
    }

    if domain == "github.com" {
      if let Some(res) = regex!("^/([^/]+)/([^/]+)").captures(link_href.path()) {
        if self.is_same_repo_as(&res[1], &res[2]).await {
          return Some(ProjectLink::Repo);
        }
      }
    }

    None
  }

  async fn comes_from_repo(&self, url: &Url) -> Option<bool> {
    let domain = url.domain()?.to_lowercase();

    if domain == "github.com" || domain == "raw.githubusercontent.com" || domain == "raw.github.com"
    {
      if let Some(res) = regex!("^/([^/]+)/([^/]+)/").captures(url.path()) {
        let comes_from_repo = self.is_same_repo_as(&res[1], &res[2]).await;
        return Some(comes_from_repo);
      }
    }

    Some(false)
  }

  fn find_keyword_mentions(&self, src: &Url, alt: Option<&str>) -> HashSet<KeywordMention> {
    let src = src.path().to_lowercase();
    let alt = alt.map(|alt| alt.to_lowercase()).unwrap_or(String::new());

    let mut mentions = HashSet::new();

    if src.contains("logo") || alt.contains("logo") {
      mentions.insert(KeywordMention::Logo);
    }

    if src.contains("banner") || alt.contains("banner") {
      mentions.insert(KeywordMention::Banner);
    }

    if src.contains(&self.repo) || alt.contains(&self.repo) {
      mentions.insert(KeywordMention::RepoName);
    }

    mentions
  }

  async fn is_same_repo_as(&self, user: &str, repo: &str) -> bool {
    let user = user.to_lowercase();
    let repo = repo.to_lowercase();
    is_same_repo((&self.owner, &self.repo), (&user, &repo)).await
  }
}
