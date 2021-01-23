use futures::StreamExt;
use lol_html::{element, HtmlRewriter, Settings};
use serde::Deserialize;
use std::error::Error;
use tokio::task::JoinHandle;
use url::Url;

#[derive(Deserialize)]
struct ManifestIcon {
  src: String,
}

#[derive(Deserialize)]
struct Manifest {
  icons: Vec<ManifestIcon>,
}

pub async fn get_icons(url: &str) -> Result<Vec<String>, Box<dyn Error>> {
  let mut result = reqwest::get(url).await?.bytes_stream();
  let site_url = Url::parse(url)?;

  let mut urls = Vec::new();
  let mut manifest_handle: Option<JoinHandle<Option<Vec<String>>>> = None;
  let mut logo = None;

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
                let url = site_url.join(&href)?;
                urls.push(url.to_string());
              }

              Ok(())
            }
          ),
          element!("header img", |el| {
            if logo.is_none() {
              if let Some(href) = el.get_attribute("src") {
                let url = site_url.join(&href)?;
                logo = Some(url.to_string());
              }
            }

            Ok(())
          }),
          element!("link[rel='manifest']", |el| {
            if let Some(href) = el.get_attribute("href") {
              let manifest_url = site_url.join(&href)?;
              manifest_handle = Some(tokio::spawn(async move {
                let manifest: Manifest = reqwest::get(manifest_url.as_str())
                  .await
                  .ok()?
                  .json()
                  .await
                  .ok()?;

                let mut urls = Vec::new();
                for icon in manifest.icons {
                  let url = manifest_url.join(&icon.src).ok()?;
                  urls.push(url.to_string());
                }

                Some(urls)
              }));
            }

            Ok(())
          }),
        ],
        ..Settings::default()
      },
      |_: &[u8]| {},
    )?;

    while let Some(data) = result.next().await {
      rewriter.write(&data?)?;
    }
  }

  if let Some(manifest) = manifest_handle {
    let manifest_urls = manifest.await?;
    if let Some(mut manifest_urls) = manifest_urls {
      urls.append(&mut manifest_urls);
    }
  }

  if let Some(logo) = logo {
    urls.push(logo);
  }

  Ok(urls)
}
