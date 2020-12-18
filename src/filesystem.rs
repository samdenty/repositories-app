use crate::IconManager;
use fuse::FileType;
use fuse_mt::FilesystemMT;
use fuse_mt::*;
use once_cell::sync::Lazy;
use std::path::Path;
use std::{env, ffi::OsStr, io::Error};
use time::Timespec;

const TTL: Lazy<Timespec> = Lazy::new(|| Timespec::new(1, 0)); // 1 second
const UNIX_EPOCH: Lazy<Timespec> = Lazy::new(|| Timespec::new(0, 0));

pub struct GitHubFS {}

impl FilesystemMT for GitHubFS {
  fn readdir(&self, _req: RequestInfo, path: &Path, fh: u64) -> ResultReaddir {
    println!("readdir {:?}", path);
    let mut entries: Vec<DirectoryEntry> = vec![];

    entries.push(DirectoryEntry {
      kind: FileType::Directory,
      name: "samdenty".into(),
    });

    Ok(entries)
  }

  fn opendir(&self, _req: RequestInfo, path: &Path, _flags: u32) -> ResultOpen {
    println!("opendir {:?}", path);

    Ok((11, 0))
  }

  fn getattr(&self, _req: RequestInfo, path: &Path, _fh: Option<u64>) -> ResultEntry {
    println!("getattr {:?}", path);

    Ok((
      *TTL,
      FileAttr {
        size: 128,
        blocks: 8,
        atime: *UNIX_EPOCH, // 1970-01-01 00:00:00
        mtime: *UNIX_EPOCH,
        ctime: *UNIX_EPOCH,
        crtime: *UNIX_EPOCH,
        kind: FileType::Directory,
        perm: 0o755,
        nlink: 4,
        uid: 501,
        gid: 20,
        rdev: 0,
        flags: 0,
      },
    ))
  }

  fn statfs(&self, _req: RequestInfo, path: &Path) -> ResultStatfs {
    println!("statfs: {:?}", path);

    Ok(Statfs {
      blocks: 0,
      bavail: 0,
      bfree: 0,
      bsize: 0,
      ffree: 0,
      files: 0,
      frsize: 0,
      namelen: 0,
    })
  }
}

pub fn mount(icon_manager: IconManager) -> Result<(), Error> {
  let options = [
    "-o",
    "rwo",
    "-o",
    "fsname=hello",
    "-o",
    "volname=GitHub",
    "-o",
    "volicon=/Users/samdenty/Downloads/25231.icns",
    "-o",
    "allow_root",
  ]
  .iter()
  .map(|o| o.as_ref())
  .collect::<Vec<&OsStr>>();
  let mountpoint = env::args_os().nth(1).unwrap();

  fuse_mt::mount(FuseMT::new(GitHubFS {}, 1), &mountpoint, &options)?;

  Ok(())
}
