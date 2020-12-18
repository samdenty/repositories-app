use byteorder::{BigEndian, WriteBytesExt};
use std::io::Error;

// A reverse-engineered implementation of the macos
// Icon\r resource fork binary format
pub fn encode(icns: &Vec<u8>) -> Result<Vec<u8>, Error> {
  let mut rsrc = Vec::new();
  let icon_size = icns.len() as u32;

  let mut header: Vec<u32> = vec![0; 65];
  header[0] = 0x100;
  header[1] = icon_size + 0x104;
  header[2] = icon_size + 0x4;
  header[3] = 0x32;
  header[64] = icon_size;

  for &n in &header {
    rsrc.write_u32::<BigEndian>(n)?;
  }
  for &n in icns {
    rsrc.write_u8(n)?;
  }
  for &n in &vec![
    0x00000100,
    icon_size + 0x104,
    icon_size + 0x4,
    0x00000032,
    0x00000000,
    0x00000000,
    0x001C0032,
    0x00006963,
    0x6E730000,
    0x000ABFB9,
    0xFFFF0000,
    0x00000000,
    0x00000000,
  ] {
    rsrc.write_u32::<BigEndian>(n)?;
  }

  Ok(rsrc)
}
