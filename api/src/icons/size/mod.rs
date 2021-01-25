mod ico;
mod jpeg;
mod png;

pub use ico::*;
pub use jpeg::*;
pub use png::*;

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::{
  cmp::Ordering,
  error::Error,
  io::{Read, Seek, SeekFrom},
  ops::{Deref, DerefMut},
};

#[derive(Deserialize, Serialize, PartialEq, Eq)]
pub struct IconSizes(Vec<IconSize>);

#[derive(Debug, PartialEq, Eq)]
pub struct IconSize {
  width: u32,
  height: u32,
}

impl Ord for IconSize {
  fn cmp(&self, other: &Self) -> Ordering {
    let self_res = self.width * self.height;
    let other_res = other.width * other.height;
    other_res.cmp(&self_res)
  }
}

impl PartialOrd for IconSize {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl IconSize {
  pub fn new(width: u32, height: u32) -> Self {
    Self { width, height }
  }
}

impl IconSizes {
  pub fn new() -> Self {
    IconSizes(Vec::new())
  }

  pub fn from_str(sizes_str: &str) -> Result<IconSizes, Box<dyn Error>> {
    let size_strs = sizes_str.split(" ");

    let mut sizes = IconSizes::new();
    for size in size_strs {
      if let Ok(size) = serde_json::from_value(Value::String(size.to_string())) {
        sizes.push(size);
      }
    }

    if sizes.is_empty() {
      return Err("must contain a size".into());
    }

    sizes.sort();

    Ok(sizes)
  }

  pub fn add_size(&mut self, width: u32, height: u32) {
    self.push(IconSize::new(width, height))
  }

  pub fn largest(&self) -> &IconSize {
    &self.0[0]
  }

  pub fn into_largest(self) -> IconSize {
    self.0.into_iter().next().unwrap()
  }
}

impl Deref for IconSizes {
  type Target = Vec<IconSize>;
  fn deref(&self) -> &Vec<IconSize> {
    &self.0
  }
}

impl DerefMut for IconSizes {
  fn deref_mut(&mut self) -> &mut Vec<IconSize> {
    &mut self.0
  }
}

impl Ord for IconSizes {
  fn cmp(&self, other: &Self) -> Ordering {
    self.largest().cmp(&other.largest())
  }
}

impl PartialOrd for IconSizes {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Serialize for IconSize {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(&format!("{}x{}", self.width, self.height))
  }
}

impl<'de> Deserialize<'de> for IconSize {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let value: String = Deserialize::deserialize(deserializer)?;

    let mut split = value.split("x");
    let width = split
      .next()
      .ok_or(de::Error::custom("expected width"))?
      .parse()
      .map_err(de::Error::custom)?;

    let height = split
      .next()
      .ok_or(de::Error::custom("expected height"))?
      .parse()
      .map_err(de::Error::custom)?;

    Ok(IconSize::new(width, height))
  }
}

fn slice_eq<T: Read + Seek + Unpin>(
  cur: &mut T,
  offset: u64,
  slice: &[u8],
) -> Result<bool, Box<dyn Error>> {
  cur.seek(SeekFrom::Start(offset))?;
  let mut buffer = vec![0; slice.len()];
  cur.read_exact(&mut buffer)?;
  Ok(buffer == slice)
}

#[macro_export]
macro_rules! assert_slice_eq {
  ($cur:expr, $offset:expr, $slice:expr, $($arg:tt)+) => {{
    if !super::slice_eq($cur, $offset, $slice)? {
      return Err(format!($($arg)+).into());
    }
  }};
}
