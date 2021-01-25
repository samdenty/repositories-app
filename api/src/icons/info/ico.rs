use byteorder::{LittleEndian, ReadBytesExt};
use futures::prelude::*;
use std::{
  error::Error,
  io::{Cursor, Seek, SeekFrom},
};

const ICO_TYPE: u16 = 1;
const INDEX_SIZE: u16 = 16;

pub async fn get_ico_sizes<R: AsyncRead + Unpin>(
  mut reader: R,
) -> Result<Vec<String>, Box<dyn Error>> {
  let mut header = [0; 6];
  reader.read_exact(&mut header).await?;
  let mut header = Cursor::new(header);

  let header_type = header.read_u16::<LittleEndian>()?;
  let icon_type = header.read_u16::<LittleEndian>()?;

  if header_type != 0 || icon_type != ICO_TYPE {
    return Err("bad header".into());
  }

  let icon_count = header.read_u16::<LittleEndian>()?;

  let mut data = vec![0; (icon_count * INDEX_SIZE) as usize];
  reader.read_exact(&mut data).await?;
  let mut data = Cursor::new(data);

  let mut sizes = Vec::new();
  for i in 0..icon_count {
    data.seek(SeekFrom::Start((INDEX_SIZE * i) as _))?;

    let width = data.read_u8()?;
    let height = data.read_u8()?;

    sizes.push(format!("{}x{}", width, height))
  }

  Ok(sizes)
}
