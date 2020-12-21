mod path;

use crate::{IconManager, Repo, User};
use fuse::FileType;
use fuse_mt::FilesystemMT;
use fuse_mt::*;
use once_cell::sync::Lazy;
use std::convert::TryInto;
use std::sync::Mutex;
use std::{env, ffi::OsStr, io::Error};
use std::{path::Path, sync::Arc};
use time::Timespec;

const TTL: Lazy<Timespec> = Lazy::new(|| Timespec::new(1, 0)); // 1 second
const UNIX_EPOCH: Lazy<Timespec> = Lazy::new(|| Timespec::new(0, 0));

const FINDER_INFO: &str = "com.apple.FinderInfo";
const RSRC_FORK: &str = "com.apple.ResourceFork";

pub struct GitHubFS {
  icon_manager: Mutex<IconManager>,
}

impl GitHubFS {
  pub fn new(icon_manager: Mutex<IconManager>) -> Self {
    Self { icon_manager }
  }
}

impl FilesystemMT for GitHubFS {
  fn readdir(&self, _req: RequestInfo, path: &Path, fh: u64) -> ResultReaddir {
    let path = path::parse(path);
    println!("readdir {:?}", path);
    let mut entries: Vec<DirectoryEntry> = vec![];

    match path {
      path::Kind::Root => {
        for name in vec!["samdenty", "microsoft", "facebook", "google", "surma"] {
          entries.push(DirectoryEntry {
            kind: FileType::Directory,
            name: name.into(),
          });
        }
      }

      path::Kind::User(login) => {
        let repos = Repo::get_for_user(&login).unwrap();

        for repo in &repos {
          entries.push(DirectoryEntry {
            kind: FileType::Directory,
            name: repo.name.clone().into(),
          })
        }

        entries.push(DirectoryEntry {
          kind: FileType::RegularFile,
          name: "Icon\r".into(),
        });
      }

      path::Kind::Repo(_, _) => {
        entries.push(DirectoryEntry {
          kind: FileType::RegularFile,
          name: "Icon\r".into(),
        });
      }
      _ => {}
    }

    Ok(entries)
  }

  fn opendir(&self, _req: RequestInfo, path: &Path, _flags: u32) -> ResultOpen {
    println!("opendir {:?}", path);

    Ok((11, 0))
  }

  fn getattr(&self, _req: RequestInfo, path: &Path, _fh: Option<u64>) -> ResultEntry {
    let path = path::parse(path);
    println!("getattr {:?}", path);

    match path {
      path::Kind::Icon(_) => Ok((
        *TTL,
        FileAttr {
          size: 13,
          blocks: 8,
          atime: *UNIX_EPOCH,
          mtime: *UNIX_EPOCH,
          ctime: *UNIX_EPOCH,
          crtime: *UNIX_EPOCH,
          kind: FileType::RegularFile,
          perm: 0o644,
          nlink: 1,
          uid: 501,
          gid: 20,
          rdev: 0,
          flags: 0,
        },
      )),

      path::Kind::Root => Ok((
        *TTL,
        FileAttr {
          size: 128,
          blocks: 8,
          atime: *UNIX_EPOCH,
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
      )),

      path::Kind::User(_) => Ok((
        *TTL,
        FileAttr {
          size: 128,
          blocks: 8,
          atime: *UNIX_EPOCH,
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
      )),

      path::Kind::Repo(_, _) => Ok((
        *TTL,
        FileAttr {
          size: 128,
          blocks: 8,
          atime: *UNIX_EPOCH,
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
      )),

      _ => Err(libc::ENOSYS),
    }
  }

  fn getxattr(&self, _req: RequestInfo, path: &Path, name: &OsStr, size: u32) -> ResultXattr {
    let path = path::parse(path);
    let name = name.to_str().ok_or(-1)?;

    println!("getxattr {:?} {:?}", path, name);

    let data: Option<Vec<u8>> = match &path {
      path::Kind::Icon(icon) => match name {
        FINDER_INFO => Some(
          hex!("69636F6E 4D414353 40100000 00000000 00000000 00000000 00000000 00000000").to_vec(),
        ),
        RSRC_FORK => {
          let mut icon_manager = self.icon_manager.lock().ok().ok_or(-1)?;
          let slug = match icon {
            path::Icon::User(login) => login.to_string(),
            path::Icon::Repo(login, repo) => format!("{}/{}", login, repo),
          };
          let icon = icon_manager
            .load(&format!("https://github.com/{}", slug))
            .unwrap();
          Some(icon.rsrc.clone())
        }
        _ => None,
      },
      path::Kind::User(_) | path::Kind::Repo(_, _) => match name {
        FINDER_INFO => Some(
          hex!("00000000 00000000 04000000 00000000 00000000 00000000 00000000 00000000").to_vec(),
        ),
        _ => None,
      },
      _ => None,
    };

    match data {
      Some(data) => Ok(match size {
        0 => Xattr::Size(data.len().try_into().unwrap()),
        _ => Xattr::Data(data),
      }),
      _ => Err(libc::ENOATTR),
    }
  }

  fn listxattr(&self, _req: RequestInfo, path: &Path, _size: u32) -> ResultXattr {
    println!("listxattr {:?}", path);

    Err(libc::ENOSYS)
  }

  fn statfs(&self, _req: RequestInfo, path: &Path) -> ResultStatfs {
    // println!("statfs: {:?}", path);

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
  let mountpoint = "./test"; //env::args_os().nth(1).unwrap();

  let icon_manager = Mutex::new(icon_manager);
  fuse_mt::mount(
    FuseMT::new(GitHubFS::new(icon_manager), 1),
    &mountpoint,
    &options,
  )?;

  Ok(())
}
