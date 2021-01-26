mod info;
mod size;

use self::info::IconInfo;
use future::join_all;
use futures::prelude::*;
use futures::StreamExt;
use lol_html::{element, HtmlRewriter, Settings};
use once_cell::sync::Lazy;
use reqwest::{header::*, Client, IntoUrl};
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, collections::HashMap, error::Error, sync::Arc};

static CLIENT: Lazy<Arc<Client>> = Lazy::new(|| {
  let mut headers = HeaderMap::new();
  headers.insert(USER_AGENT, HeaderValue::from_str("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/88.0.4324.104 Safari/537.36").unwrap());
  let client = Client::builder().default_headers(headers).build().unwrap();
  Arc::new(client)
});

#[derive(Serialize, Clone, PartialOrd, PartialEq, Ord, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IconKind {
  SiteLogo,
  AppIcon,
  Favicon,
}

#[derive(Serialize, PartialEq, Eq)]
pub struct Icon {
  pub url: String,
  pub kind: IconKind,
  #[serde(flatten)]
  pub info: IconInfo,
}

impl Ord for Icon {
  fn cmp(&self, other: &Self) -> Ordering {
    self.info.cmp(&other.info)
  }
}

impl PartialOrd for Icon {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

pub async fn get_icons<U: IntoUrl>(url: U) -> Result<Vec<Icon>, Box<dyn Error>> {
  let res = CLIENT.get(url).header(ACCEPT, "text/html").send().await?;
  let url = res.url().clone();
  let mut body = res.bytes_stream();

  let mut icons = HashMap::new();
  let mut logo = None;
  let mut manifest = None;

  let mut found_favicon = false;

  {
    let mut rewriter = HtmlRewriter::try_new(
      Settings {
        element_content_handlers: vec![
          element!(
            concat!(
              "link[rel='icon'],",
              "link[rel='shortcut icon'],",
              "link[rel='apple-touch-icon'],",
              "link[rel='apple-touch-icon-precomposed']"
            ),
            |el| {
              if let Some(href) = el.get_attribute("href") {
                found_favicon = true;
                let url = url.join(&href)?;
                if !icons.contains_key(&url) {
                  let info = IconInfo::get(url.clone(), el.get_attribute("sizes"));
                  icons.insert(url, (IconKind::Favicon, info));
                }
              }

              Ok(())
            }
          ),
          element!(
            concat!(
              "header img,",
              "img[src*=logo],",
              "img[alt*=logo],",
              "img[class*=logo]"
            ),
            |el| {
              if logo.is_some() {
                return Ok(());
              };

              if let Some(href) = el.get_attribute("src") {
                let url = url.join(&href)?;
                let info = IconInfo::get(url.clone(), None);
                logo = Some((url, info));
              }

              Ok(())
            }
          ),
          element!("link[rel='manifest']", |el| {
            if let Some(href) = el.get_attribute("href") {
              let manifest_url = url.join(&href)?;
              let client = CLIENT.clone();

              manifest = Some(async move {
                #[derive(Deserialize)]
                struct ManifestIcon {
                  src: String,
                  sizes: Option<String>,
                }

                #[derive(Deserialize)]
                struct Manifest {
                  icons: Vec<ManifestIcon>,
                }

                let manifest: Manifest = client
                  .get(manifest_url.as_str())
                  .send()
                  .await
                  .ok()?
                  .json()
                  .await
                  .ok()?;

                let mut icons = HashMap::new();
                for icon in manifest.icons {
                  let url = manifest_url.join(&icon.src).ok()?;
                  if !icons.contains_key(&url) {
                    let info = IconInfo::get(url.clone(), icon.sizes);
                    icons.insert(url, (IconKind::AppIcon, info));
                  }
                }

                Some(icons)
              });
            }

            Ok(())
          }),
        ],
        ..Settings::default()
      },
      |_: &[u8]| {},
    )?;

    while let Some(data) = body.next().await {
      rewriter.write(&data?)?;
    }
  }

  // Check for default favicon.ico
  if !found_favicon {
    let url = url.join("/favicon.ico")?;
    if !icons.contains_key(&url) {
      let info = IconInfo::get(url.clone(), None);
      icons.insert(url, (IconKind::Favicon, info));
    }
  }

  if let Some((url, info)) = logo {
    if !icons.contains_key(&url) {
      icons.insert(url, (IconKind::SiteLogo, info));
    }
  }

  if let Some(manifest) = manifest {
    let manifest_icons = manifest.await;
    if let Some(manifest_icons) = manifest_icons {
      icons.extend(manifest_icons);
    }
  }

  let (urls, infos): (Vec<_>, Vec<_>) = icons
    .into_iter()
    .map(|(url, (kind, info))| ((url, kind), info))
    .unzip();

  let mut icons = Vec::new();
  for (i, info) in join_all(infos).await.into_iter().enumerate() {
    match info {
      Ok(info) => {
        let (url, kind) = urls.get(i).unwrap();

        icons.push(Icon {
          url: url.to_string(),
          kind: kind.clone(),
          info,
        })
      }
      Err(e) => {
        warn!("failed to parse icon: {}", e);
      }
    }
  }

  icons.sort();

  Ok(icons)
}
