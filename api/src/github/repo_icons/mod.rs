use crate::github_api_get;
use regex::{escape, Regex};
use reqwest::Url;
use scraper::{ElementRef, Html, Selector};
use std::error::Error;

#[derive(Debug, Clone)]
enum PrimaryHeadingPos {
  Preceding,
  Trailing,
}

struct PrimaryHeading<'a> {
  primary_heading: Option<ElementRef<'a>>,
  next_heading: Option<ElementRef<'a>>,
  first_img: Option<Option<PrimaryHeadingPos>>,
}

impl<'a> PrimaryHeading<'a> {
  pub fn new(document: &'a Html) -> Self {
    let selector = Selector::parse("h1, h2, h3, hr").unwrap();
    let mut headings = document.select(&selector);
    let first_heading = headings.next();
    let second_heading = headings.next();

    Self {
      primary_heading: first_heading,
      next_heading: second_heading,
      first_img: None,
    }
  }

  pub fn contains(&mut self, element: ElementRef) -> bool {
    let pos = self
      .primary_heading
      .map(|primary| {
        // if its before the primary heading
        if primary.id() > element.id() {
          Some(PrimaryHeadingPos::Preceding)
        } else {
          // or if its between the primary & next heading
          if self
            .next_heading
            .map(|next| next.id() > element.id())
            .unwrap_or(true)
          {
            // but theres an image already before us...
            if let Some(_) = &self.first_img {
              // then its out-of-bounds
              None
            } else {
              // else its trailing
              Some(PrimaryHeadingPos::Trailing)
            }
          } else {
            // if after both the first & second headings,
            // then its out-of-bounds
            None
          }
        }
      })
      .unwrap_or(Some(PrimaryHeadingPos::Preceding));

    if self.first_img.is_none() {
      self.first_img = Some(pos.clone());
    }

    pos.is_some()
  }
}

#[derive(Debug)]
enum LinksTo {
  Website,
  Repo,
}

#[derive(Debug)]
struct Image {
  src: String,
  in_primary_heading: bool,
  at_edge_of_primary_heading: bool,
  is_center_aligned: bool,
  has_size_attr: bool,
  links_to: Option<LinksTo>,
  sourced_from_repo: bool,
}

// pub struct RepoIcons<'a> {
//   owner: &'a str,
//   repo: &'a str,
//   homepage: Option<Url>,
//   primary_heading: PrimaryHeading<'a>,
//   document: Html,
// }

// impl<'a> RepoIcons<'a> {
//   pub async fn new(
//     owner: &'a str,
//     repo: &'a str,
//     homepage: Option<Url>,
//   ) -> Result<RepoIcons<'a>, Box<dyn Error>> {
//     let body = github_api_get!("repos/{}/{}/readme", owner, repo)
//       .header("Accept", "application/vnd.github.html")
//       .send()
//       .await?
//       .text()
//       .await?;

//     let document = Html::parse_document(&body);
//     let mut primary_heading = PrimaryHeading::new(&document);

//     Ok(Self {
//       owner,
//       repo,
//       homepage,
//       primary_heading,
//       document,
//     })
//   }
// }

pub async fn get_repo_icons(
  owner: &str,
  repo: &str,
  homepage: Option<Url>,
) -> Result<(), Box<dyn Error>> {
  let body = github_api_get!("repos/{}/{}/readme", owner, repo)
    .header("Accept", "application/vnd.github.html")
    .send()
    .await?
    .text()
    .await?;

  let readme_blob = Url::parse(&format!(
    "https://github.com/{}/{}/blob/master/",
    owner, repo
  ))?;

  let readme_raw = Url::parse(&format!(
    "https://github.com/{}/{}/raw/master/",
    owner, repo
  ))?;

  let document = Html::parse_document(&body);
  let mut primary_heading = PrimaryHeading::new(&document);

  let mut images = Vec::new();
  for element_ref in document.select(&Selector::parse("img[src]").unwrap()) {
    let elem = element_ref.value();

    let src = readme_raw.join(
      elem
        .attr("data-canonical-src")
        .or(elem.attr("src"))
        .unwrap(),
    )?;

    let cdn_src = elem.attr("data-canonical-src").and(elem.attr("src"));
    let alt = elem.attr("alt");

    let in_primary_heading = primary_heading.contains(element_ref);

    info!("{:?}", src);

    let mut is_center_aligned = false;
    let mut link = None;

    for element_ref in element_ref.ancestors().map(ElementRef::wrap).flatten() {
      let element = element_ref.value();

      if element.attr("align") == Some("center") {
        is_center_aligned = true;
      }

      if element.name() == "a" && link.is_none() {
        link = Some(element);
      }
    }

    let sourced_from_repo: bool = {
      let domain = src.domain().ok_or("no domain")?;

      let raw_url = domain == "github.com"
        && Regex::new(&format!(r"^/{}/{}/raw/", escape(owner), escape(repo)))
          .unwrap()
          .is_match(src.path());

      let raw_cdn_url = domain == "raw.githubusercontent.com"
        && Regex::new(&format!(r"^/{}/{}/", escape(owner), escape(repo)))
          .unwrap()
          .is_match(src.path());

      raw_cdn_url || raw_url
    };

    let links_to = {
      let link = link.ok_or("no link")?;
      let href = link.attr("href").ok_or("no href")?;

      if readme_raw.join(href)? == src {
        None
      } else {
        let url = readme_blob.join(href)?;
        let domain = url.domain().ok_or("no domain")?;

        let pages_url = Regex::new(&format!(r"^{}\.github\.(com|io)$", escape(owner)))
          .unwrap()
          .is_match(domain)
          && Regex::new(&format!(r"^/{}[?/#]", escape(repo)))
            .unwrap()
            .is_match(url.path());

        let homepage_url = homepage
          .clone()
          .map(|h| h.domain() == Some(domain))
          .unwrap_or(false);

        let repo_url = domain == "github.com"
          && Regex::new(&format!(r"^/{}/{}[?/#]", escape(owner), escape(repo)))
            .unwrap()
            .is_match(url.path());

        if pages_url || homepage_url {
          Some(LinksTo::Website)
        } else if repo_url {
          Some(LinksTo::Repo)
        } else {
          None
        }
      }
    };

    let has_size_attr = elem.attr("width").or(elem.attr("height")).is_some();

    images.push(Image {
      src: src.to_string(),
      at_edge_of_primary_heading: false,
      has_size_attr,
      in_primary_heading,
      sourced_from_repo,
      links_to,
      is_center_aligned,
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
      image.at_edge_of_primary_heading = true;
    };
  }

  warn!("{:#?}", images);

  // let mut weight = 0;

  // // if not in the primary heading,
  // // don't take into account basic heuristics
  // if primary_heading_pos.is_some() {
  //   if is_center_aligned {
  //     weight += 2;
  //   }

  //   if has_size {
  //     weight += 1;
  //   }

  //   if link.is_some() {
  //     weight += 1;
  //   }
  // }
  Ok(())
}
