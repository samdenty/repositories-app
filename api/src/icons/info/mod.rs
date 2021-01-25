mod ico;
mod png;

use super::CLIENT;
use data_url::DataUrl;
use futures::{io::Cursor, prelude::*, stream::TryStreamExt};
use ico::get_ico_sizes;
use mime::*;
use png::get_png_sizes;
use reqwest::{header::*, Url};
use serde::{Deserialize, Serialize};
use std::{
  error::Error,
  io::{self, Read, Seek, SeekFrom},
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum IconKind {
  PNG,
  ICO,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum IconInfo {
  PNG { size: String },
  ICO { sizes: Vec<String> },
  SVG,
}

pub async fn get_info(url: Url, sizes: Option<String>) -> Result<IconInfo, Box<dyn Error>> {
  let (mime, body): (_, Box<dyn AsyncRead + Unpin>) = match url.scheme() {
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
    (IMAGE, PNG) => {
      if let Some(size) = sizes {
        return Ok(IconInfo::PNG { size });
      }
      IconKind::PNG
    }

    (IMAGE, "x-icon") | (IMAGE, "vnd.microsoft.icon") => {
      if let Some(sizes) = sizes {
        let sizes = sizes.split(" ").map(|s| s.to_string()).collect();
        return Ok(IconInfo::ICO { sizes });
      }

      IconKind::ICO
    }

    (IMAGE, SVG) => return Ok(IconInfo::SVG),

    _ => return Err("unsupported mime type".into()),
  };

  Ok(match kind {
    IconKind::PNG => {
      let size = get_png_sizes(body).await?;
      IconInfo::PNG { size }
    }
    IconKind::ICO => {
      let sizes = get_ico_sizes(body).await?;
      IconInfo::ICO { sizes }
    }
  })
}

fn slice_eq<T>(cur: &mut T, offset: u64, slice: &[u8]) -> Result<bool, Box<dyn Error>>
where
  T: Read + Seek + Unpin,
{
  cur.seek(SeekFrom::Start(offset))?;
  let mut buffer = vec![0; slice.len()];
  cur.read_exact(&mut buffer)?;
  Ok(buffer == slice)
}
