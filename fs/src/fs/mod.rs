mod path;

use chrono::{DateTime, Local};
use colored::Colorize;
use fuse::FileType;
use fuse_mt::*;
use once_cell::sync::Lazy;
use relative_path::RelativePath;
use std::convert::TryInto;
use std::sync::Mutex;
use std::{env, error::Error, ffi::OsStr};
use std::{path::Path, sync::Arc};
use time::Timespec;

use crate::{
  github::{Branch, Repo, Tree, UserRepos},
  icon_manager::IconManager,
};

static TTL: Lazy<Timespec> = Lazy::new(|| Timespec::new(1, 0)); // 1 second
static UNIX_EPOCH: Lazy<Timespec> = Lazy::new(|| Timespec::new(0, 0));

const FINDER_INFO: &str = "com.apple.FinderInfo";
const RSRC_FORK: &str = "com.apple.ResourceFork";

const FINDER_INFO_HIDDEN: [u8; 32] =
  hex!("69636F6E 4D414353 40100000 00000000 00000000 00000000 00000000 00000000");
const FINDER_INFO_HAS_ICON: [u8; 32] =
  hex!("00000000 00000000 04000000 00000000 00000000 00000000 00000000 00000000");

pub struct GitHubFS {
  icon_manager: IconManager,
}

impl GitHubFS {
  pub fn new(icon_manager: IconManager) -> Self {
    Self { icon_manager }
  }
}

#[async_trait]
impl FilesystemMT for GitHubFS {
  async fn readdir(&self, _req: RequestInfo, path: &Path, fh: u64) -> ResultReaddir {
    let path = path::parse(path).map_err(|_| libc::ENOENT)?;
    debug!("{} {:?}", "readdir".bright_yellow(), path);
    let mut entries: Vec<DirectoryEntry> = vec![];

    match path {
      path::Kind::Root => {
        for name in vec![
          "samdenty",
          "microsoft",
          "facebook",
          "google",
          "surma",
          "samyk",
          "sindresorhus",
        ] {
          entries.push(DirectoryEntry {
            kind: FileType::Directory,
            name: name.into(),
          });
        }
      }

      path::Kind::User(owner) => {
        let repos = UserRepos::get(owner).await.map_err(|_| libc::ENOENT)?;

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

      path::Kind::DefaultTree(owner, repo, path) => {
        let branch = Branch::get_default(owner, repo)
          .await
          .map_err(|_| libc::ENOENT)?;
        let tree = branch.get_dir(&path).await.map_err(|_| libc::ENOENT)?;

        let path = RelativePath::new(&path);
        for entry in tree.entries {
          entries.push(DirectoryEntry {
            kind: (&entry).into(),
            name: path.relative(entry.path).to_string().into(),
          })
        }

        entries.push(DirectoryEntry {
          kind: FileType::RegularFile,
          name: "Icon\r".into(),
        });
      }

      path::Kind::CustomTree(owner, repo, path) => {}

      path::Kind::Icon(_) => {}
    }

    Ok(entries)
  }

  async fn opendir(&self, _req: RequestInfo, path: &Path, _flags: u32) -> ResultOpen {
    debug!("opendir {:?}", path);

    Ok((11, 0))
  }

  async fn open(&self, _req: RequestInfo, path: &Path, _flags: u32) -> ResultOpen {
    debug!("open {:?}", path);

    Ok((11, 0))
  }

  async fn getattr(&self, _req: RequestInfo, path: &Path, _fh: Option<u64>) -> ResultEntry {
    let path = path::parse(path).map_err(|_| libc::ENOENT)?;
    debug!("{} {:?}", "getattr".magenta(), path);

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

      path::Kind::DefaultTree(owner, repo, path) => {
        if path == "" {
          return Ok((
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
          ));
        }

        let branch = Branch::get_default(owner, repo)
          .await
          .map_err(|_| libc::ENOENT)?;
        let entry = branch.get_entry(&path).await.map_err(|_| libc::ENOENT)?;
        let blob = entry.blob(owner, repo).await.unwrap();

        Ok((
          *TTL,
          FileAttr {
            size: blob.map(|b| b.size as _).unwrap_or(0),
            blocks: 8,
            atime: *UNIX_EPOCH,
            mtime: *UNIX_EPOCH,
            ctime: *UNIX_EPOCH,
            crtime: *UNIX_EPOCH,
            kind: (&entry).into(),
            perm: 0o755,
            nlink: 4,
            uid: 501,
            gid: 20,
            rdev: 0,
            flags: 0,
          },
        ))
      }

      _ => Err(libc::ENOSYS),
    }
  }

  async fn getxattr(&self, _req: RequestInfo, path: &Path, name: &OsStr, size: u32) -> ResultXattr {
    let path = path::parse(path).map_err(|_| libc::ENOENT)?;
    let name = name.to_str().ok_or(-1)?;

    let data: Option<Vec<u8>> = match &path {
      path::Kind::Icon(icon) => match name {
        FINDER_INFO => Some(FINDER_INFO_HIDDEN.to_vec()),
        RSRC_FORK => match icon {
          path::Icon::User(user) => {
            // let mut icon_manager = self.icon_manager.lock().ok().ok_or(-1)?;

            let icon = self
              .icon_manager
              .load_repo(&format!("http://127.0.0.1:8080/a.html?repo={}", user,));
            icon.ok().map(|icon| icon.rsrc.clone())
            // Some(Vec::new())
          }
          path::Icon::Repo(user, repo) => {
            // let mut icon_manager = self.icon_manager.lock().ok().ok_or(-1)?;
            // let icon = self
            //   .icon_manager
            //   .load_repo(&format!("https://github.com/{}/{}", user, repo));
            let icon = self
              .icon_manager
              .load_repo(&format!("http://127.0.0.1:8080/a.html?repo={}", user,));

            icon.ok().map(|icon| icon.rsrc.clone())
            // Some(Vec::new())
          }
        },
        _ => None,
      },
      path::Kind::User(_) | path::Kind::DefaultTree(_, _, _) => match name {
        FINDER_INFO => Some(FINDER_INFO_HAS_ICON.to_vec()),
        _ => None,
      },
      _ => None,
    };

    match data {
      Some(data) => {
        debug!("{} {:?} {:?} {}", "getxattr".blue(), path, name, data.len());
        Ok(match size {
          0 => Xattr::Size(data.len().try_into().unwrap()),
          _ => Xattr::Data(data),
        })
      }
      _ => Err(libc::ENOATTR),
    }
  }

  async fn listxattr(&self, _req: RequestInfo, path: &Path, _size: u32) -> ResultXattr {
    debug!("{} {:?}", "listxattr".bright_purple(), path);

    Err(libc::ENOSYS)
  }

  async fn statfs(&self, _req: RequestInfo, path: &Path) -> ResultStatfs {
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

  async fn read(
    &self,
    _req: RequestInfo,
    path: &Path,
    _fh: u64,
    offset: u64,
    size: u32,
  ) -> ResultData {
    let path = path::parse(path).map_err(|_| libc::ENOENT)?;
    debug!("read: {:?} {:#x} @ {:#x}", path, size, offset);

    match path {
      path::Kind::DefaultTree(owner, repo, path) => {
        let branch = Branch::get_default(owner, repo)
          .await
          .map_err(|_| libc::ENOENT)?;
        let tree = branch.get_entry(&path).await.map_err(|_| libc::ENOENT)?;
        let blob = tree.blob(owner, repo).await.unwrap().ok_or(libc::EISDIR)?;

        let data = blob.get_data(owner, repo).await.map_err(|_| libc::ENOENT)?;

        Ok(data[offset as usize..offset as usize + size as usize].to_vec())
      }

      _ => Err(libc::ENOSYS),
    }
  }
}

pub fn mount(icon_manager: IconManager) -> Result<(), Box<dyn Error>> {
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

  // let icon_manager = Mutex::new(icon_manager);
  fuse_mt::mount(
    FuseMT::new(GitHubFS::new(icon_manager), 10),
    &mountpoint,
    &options,
  )?;

  Ok(())
}
