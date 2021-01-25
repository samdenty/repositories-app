use super::slice_eq;
use byteorder::{BigEndian, ReadBytesExt};
use futures::prelude::*;
use std::{error::Error, io::Cursor};

pub async fn get_png_sizes<T>(mut reader: T) -> Result<String, Box<dyn Error>>
where
  T: AsyncRead + Unpin,
{
  let mut header = vec![0; 24];
  reader.read_exact(&mut header).await?;
  let header = &mut Cursor::new(header);

  if !slice_eq(header, 0, b"\x89PNG\r\n\x1a\n")? || !slice_eq(header, 12, b"IHDR")? {
    return Err("bad header".into());
  };

  let width = header.read_u32::<BigEndian>()?;
  let height = header.read_u32::<BigEndian>()?;

  Ok(format!("{}x{}", width, height))
}
