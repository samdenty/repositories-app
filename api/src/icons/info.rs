use super::size::*;
use super::CLIENT;
use data_url::DataUrl;
use futures::{io::Cursor, prelude::*, stream::TryStreamExt};
use mime::MediaType;
use reqwest::{header::*, Url};
use serde::{Deserialize, Serialize};
use std::{
  cmp::Ordering,
  error::Error,
  io::{self},
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum IconType {
  PNG,
  JPEG,
  ICO,
}

#[derive(Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum IconInfo {
  PNG { size: IconSize },
  JPEG { size: IconSize },
  ICO { sizes: IconSizes },
  SVG,
}

impl Ord for IconInfo {
  fn cmp(&self, other: &Self) -> Ordering {
    let this_size = self.size();
    let other_size = other.size();

    if this_size.is_none() && other_size.is_none() {
      Ordering::Equal
    } else if let (Some(this_size), Some(other_size)) = (this_size, other_size) {
      this_size.cmp(other_size)
    } else if this_size.is_none() {
      Ordering::Less
    } else {
      Ordering::Greater
    }
  }
}

impl PartialOrd for IconInfo {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl IconInfo {
  pub async fn get(url: Url, sizes: Option<String>) -> Result<IconInfo, Box<dyn Error>> {
    let sizes = sizes.as_ref().and_then(|s| IconSizes::from_str(s).ok());

    let (mime, mut body): (_, Box<dyn AsyncRead + Unpin>) = match url.scheme() {
      "data" => {
        let url = url.to_string();
        let url = DataUrl::process(&url).map_err(|_| "failed to parse data uri")?;

        let mime = url.mime_type().to_string().parse::<MediaType>()?;

        let body = Cursor::new(
          url
            .decode_to_vec()
            .map_err(|_| "failed to decode data uri body")?
            .0,
        );

        (mime, Box::new(body))
      }

      _ => {
        let res = CLIENT.get(url).send().await?;
        if !res.status().is_success() {
          return Err("failed to fetch".into());
        };

        let mime = res
          .headers()
          .get(CONTENT_TYPE)
          .ok_or("no content type")?
          .to_str()?
          .parse::<MediaType>()?;

        let body = res
          .bytes_stream()
          .map(|result| {
            result.map_err(|error| io::Error::new(io::ErrorKind::Other, error.to_string()))
          })
          .into_async_read();

        (mime, Box::new(body))
      }
    };

    let kind = match (mime.type_(), mime.subtype()) {
      (mime::IMAGE, mime::PNG) => {
        if let Some(size) = sizes.map(|s| s.into_largest()) {
          return Ok(IconInfo::PNG { size });
        }
        IconType::PNG
      }

      (mime::IMAGE, mime::JPEG) => {
        if let Some(size) = sizes.map(|s| s.into_largest()) {
          return Ok(IconInfo::JPEG { size });
        }
        IconType::JPEG
      }

      (mime::IMAGE, "x-icon") | (mime::IMAGE, "vnd.microsoft.icon") => {
        if let Some(sizes) = sizes {
          return Ok(IconInfo::ICO { sizes });
        }

        IconType::ICO
      }

      (mime::IMAGE, mime::SVG) => return Ok(IconInfo::SVG),

      _ => return Err(format!("unsupported mime type {}", mime).into()),
    };

    Ok(match kind {
      IconType::PNG => {
        let size = get_png_sizes(&mut body).await?;
        IconInfo::PNG { size }
      }
      IconType::ICO => {
        let sizes = get_ico_sizes(&mut body).await?;
        IconInfo::ICO { sizes }
      }
      IconType::JPEG => {
        let size = get_jpeg_size(&mut body).await?;
        IconInfo::JPEG { size }
      }
    })
  }

  pub fn size(&self) -> Option<&IconSize> {
    match self {
      IconInfo::ICO { sizes } => Some(sizes.largest()),
      IconInfo::PNG { size } | IconInfo::JPEG { size } => Some(size),
      IconInfo::SVG => None,
    }
  }
}
