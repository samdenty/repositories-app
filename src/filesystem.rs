use crate::client;
use crate::icon_manager::IconManager;
use crate::Organization;
use colored::*;
use fuse::*;
use github_rs::client::Executor;
use libc::*;
use std::io::Error;
use std::time::SystemTime;
use std::time::{Duration, UNIX_EPOCH};
use std::{convert::TryInto, path::Path};
use std::{env, ffi::OsStr};

const TTL: Duration = Duration::from_secs(1); // 1 second

struct HelloFS {
  orgs: Vec<Organization>,
  icon_manager: IconManager,
}

impl HelloFS {
  fn new(icon_manager: IconManager) -> Result<Self, Error> {
    let (headers, status, orgs) = client
      .get()
      .user()
      .orgs()
      .execute::<Vec<Organization>>()
      .unwrap();

    Ok(HelloFS {
      orgs: orgs.unwrap(),
      icon_manager,
    })
  }
}

const ROOT: FileAttr = FileAttr {
  ino: 1,
  size: 128,
  blocks: 8,
  atime: UNIX_EPOCH, // 1970-01-01 00:00:00
  mtime: UNIX_EPOCH,
  ctime: UNIX_EPOCH,
  crtime: UNIX_EPOCH,
  kind: FileType::Directory,
  perm: 0o755,
  nlink: 4,
  uid: 501,
  gid: 20,
  rdev: 0,
  flags: 0,
};
const TEST: FileAttr = FileAttr {
  ino: 2,
  size: 96,
  blocks: 8,
  atime: UNIX_EPOCH, // 1970-01-01 00:00:00
  mtime: UNIX_EPOCH,
  ctime: UNIX_EPOCH,
  crtime: UNIX_EPOCH,
  kind: FileType::Directory,
  perm: 0o755,
  nlink: 3,
  uid: 501,
  gid: 20,
  rdev: 0,
  flags: 0,
};
const ICON: FileAttr = FileAttr {
  ino: 3,
  size: 13,
  blocks: 8,
  atime: UNIX_EPOCH, // 1970-01-01 00:00:00
  mtime: UNIX_EPOCH,
  ctime: UNIX_EPOCH,
  crtime: UNIX_EPOCH,
  kind: FileType::RegularFile,
  perm: 0o644,
  nlink: 1,
  uid: 501,
  gid: 20,
  rdev: 0,
  flags: 0,
};

impl Filesystem for HelloFS {
  fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
    if name == "Icon\r" {
      println!("lookup {:?}", name.to_str());
      reply.entry(&TTL, &ICON, 0);
    } else if name == "test" {
      println!("lookup {:?}", name.to_str());
      reply.entry(&TTL, &TEST, 0);
    } else {
      println!("{} lookup {:?} {}", "enoent".red(), name.to_str(), parent);
      reply.error(ENOENT);
    }

    // if parent == 1 {
    //   let index = self
    //     .orgs
    //     .iter()
    //     .position(|org| org.login == name.to_str().unwrap());
    //   if let Some(index) = index {
    //     reply.entry(
    //       &TTL,
    //       &FileAttr {
    //         ino: (index + 2).try_into().unwrap(),
    //         size: 0,
    //         blocks: 0,
    //         atime: UNIX_EPOCH, // 1970-01-01 00:00:00
    //         mtime: UNIX_EPOCH,
    //         ctime: UNIX_EPOCH,
    //         crtime: UNIX_EPOCH,
    //         kind: FileType::Directory,
    //         perm: 0o755,
    //         nlink: 1,
    //         uid: 501,
    //         gid: 20,
    //         rdev: 0,
    //         flags: 0,
    //       },
    //       0,
    //     );
    //     return;
    //   }
    // }
  }

  fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
    // let org = if ino >= 2 {
    //   self.orgs.get((ino - 2) as usize)
    // } else {
    //   None
    // };

    if ino == 1 {
      println!("getattr: {}", ino);
      reply.attr(&TTL, &ROOT);
    } else if ino == 2 {
      println!("getattr: {}", ino);
      reply.attr(&TTL, &TEST);
    } else if ino == 3 {
      println!("getattr: {}", ino);
      reply.attr(&TTL, &ICON);
    } else {
      println!("{} getattr: {}", "enoent".red(), ino);
      reply.error(ENOENT)
    }
  }

  fn readdir(
    &mut self,
    _req: &Request,
    ino: u64,
    _fh: u64,
    offset: i64,
    mut reply: ReplyDirectory,
  ) {
    if ino == 1 {
      println!("readdir {} {}", ino, _fh);
      let entries = vec![
        (1, FileType::Directory, "."),
        (1, FileType::Directory, ".."),
        (2, FileType::Directory, "test"),
      ];
      // for (i, org) in self.orgs.iter().enumerate() {
      //   entries.push(((i + 2) as u64, FileType::Directory, org.login.as_str()));
      //   // println!("{:#?}", org.login);
      // }
      for (i, entry) in entries.into_iter().enumerate().skip(offset as usize) {
        // i + 1 means the index of the next entry
        reply.add(entry.0, (i + 1) as i64, entry.1, entry.2);
      }
      reply.ok();
    } else if ino == 2 {
      println!("readdir {} {}", ino, _fh);
      let entries = vec![
        (2, FileType::Directory, "."),
        (1, FileType::Directory, ".."),
        (3, FileType::RegularFile, "Icon\r"),
      ];
      for (i, entry) in entries.into_iter().enumerate().skip(offset as usize) {
        // i + 1 means the index of the next entry
        reply.add(entry.0, (i + 1) as i64, entry.1, entry.2);
      }
      reply.ok();
    } else {
      println!("{} readdir {} {}", "enoent".red(), ino, _fh);
      reply.error(ENOENT);
    }
  }

  fn getxattr(
    &mut self,
    _req: &Request<'_>,
    ino: u64,
    _name: &OsStr,
    _size: u32,
    reply: ReplyXattr,
  ) {
    let data = if ino == 2 {
      if _name == "com.apple.FinderInfo" {
        Some(
          hex_literal::hex!(
            "00000000 00000000 04000000 00000000 00000000 00000000 00000000 00000000"
          )
          .to_vec(),
        )
      } else {
        None
      }
    } else if ino == 3 {
      Some(if _name == "com.apple.FinderInfo" {
        hex_literal::hex!("69636F6E 4D414353 40100000 00000000 00000000 00000000 00000000 00000000")
          .to_vec()
      } else {
        let icon = self.icon_manager.load("https://google.com").unwrap();
        icon.rsrc.clone()
      })
    } else {
      None
    };

    if let Some(data) = data {
      println!("getxattr {} {:?} {}", ino, _name, _size);
      if _size == 0 {
        reply.size(data.len().try_into().unwrap());
      } else {
        reply.data(&data)
      }
    } else {
      println!("{} getxattr {} {:?} {}", "enoattr".red(), ino, _name, _size);
      reply.error(ENOATTR)
    }
  }

  // fn read(
  //   &mut self,
  //   _req: &Request,
  //   ino: u64,
  //   _fh: u64,
  //   offset: i64,
  //   _size: u32,
  //   reply: ReplyData,
  // ) {
  //   println!("read {}", ino);

  //   if ino == 10 {
  //     reply.data(&HELLO_TXT_CONTENT.as_bytes()[offset as usize..]);
  //   } else {
  //     reply.error(ENOENT);
  //   }
  // }

  fn listxattr(&mut self, _req: &Request<'_>, ino: u64, _size: u32, reply: ReplyXattr) {
    println!("listxattr {} {}", ino, _size);

    let data = if ino == 2 {
      "com.apple.FinderInfo\u{0}"
    } else if ino == 3 {
      "com.apple.FinderInfo\u{0}com.apple.ResourceFork\u{0}"
    } else {
      ""
    }
    .as_bytes();

    if _size == 0 {
      reply.size(data.len().try_into().unwrap());
    } else {
      reply.data(&data)
    }
  }

  // fn destroy(&mut self, _req: &Request<'_>) {
  //   println!("{}", "destroy".red());
  // }

  // fn forget(&mut self, _req: &Request<'_>, _ino: u64, _nlookup: u64) {
  //   println!("{}", "forget".red());
  // }

  // fn setattr(
  //   &mut self,
  //   _req: &Request<'_>,
  //   _ino: u64,
  //   _mode: Option<u32>,
  //   _uid: Option<u32>,
  //   _gid: Option<u32>,
  //   _size: Option<u64>,
  //   _atime: Option<SystemTime>,
  //   _mtime: Option<SystemTime>,
  //   _fh: Option<u64>,
  //   _crtime: Option<SystemTime>,
  //   _chgtime: Option<SystemTime>,
  //   _bkuptime: Option<SystemTime>,
  //   _flags: Option<u32>,
  //   reply: ReplyAttr,
  // ) {
  //   println!("{}", "setattr".red());
  //   reply.error(ENOSYS);
  // }

  // fn readlink(&mut self, _req: &Request<'_>, _ino: u64, reply: ReplyData) {
  //   println!("{}", "readlink".red());
  //   reply.error(ENOSYS);
  // }

  // fn mknod(
  //   &mut self,
  //   _req: &Request<'_>,
  //   _parent: u64,
  //   _name: &OsStr,
  //   _mode: u32,
  //   _rdev: u32,
  //   reply: ReplyEntry,
  // ) {
  //   println!("{}", "mknod".red());
  //   reply.error(ENOSYS);
  // }

  // fn mkdir(
  //   &mut self,
  //   _req: &Request<'_>,
  //   _parent: u64,
  //   _name: &OsStr,
  //   _mode: u32,
  //   reply: ReplyEntry,
  // ) {
  //   println!("{}", "mkdir".red());
  //   reply.entry(&TTL, &HELLO_TXT_ATTR, 0)
  //   // reply.error(ENOSYS);
  // }

  // fn unlink(&mut self, _req: &Request<'_>, _parent: u64, _name: &OsStr, reply: ReplyEmpty) {
  //   println!("{}", "unlink".red());
  //   reply.error(ENOSYS);
  // }

  // fn rmdir(&mut self, _req: &Request<'_>, _parent: u64, _name: &OsStr, reply: ReplyEmpty) {
  //   println!("{}", "rmdir".red());
  //   reply.error(ENOSYS);
  // }

  // fn symlink(
  //   &mut self,
  //   _req: &Request<'_>,
  //   _parent: u64,
  //   _name: &OsStr,
  //   _link: &Path,
  //   reply: ReplyEntry,
  // ) {
  //   println!("{}", "symlink".red());
  //   reply.error(ENOSYS);
  // }

  // fn rename(
  //   &mut self,
  //   _req: &Request<'_>,
  //   _parent: u64,
  //   _name: &OsStr,
  //   _newparent: u64,
  //   _newname: &OsStr,
  //   reply: ReplyEmpty,
  // ) {
  //   println!("{}", "rename".red());
  //   reply.error(ENOSYS);
  // }

  // fn link(
  //   &mut self,
  //   _req: &Request<'_>,
  //   _ino: u64,
  //   _newparent: u64,
  //   _newname: &OsStr,
  //   reply: ReplyEntry,
  // ) {
  //   println!("{}", "link".red());
  //   reply.error(ENOSYS);
  // }

  // fn open(&mut self, _req: &Request<'_>, _ino: u64, _flags: u32, reply: ReplyOpen) {
  //   println!("open {}", _ino);
  //   reply.opened(0, 0);
  // }

  // fn write(
  //   &mut self,
  //   _req: &Request<'_>,
  //   _ino: u64,
  //   _fh: u64,
  //   _offset: i64,
  //   _data: &[u8],
  //   _flags: u32,
  //   reply: ReplyWrite,
  // ) {
  //   println!("{}", "write".red());
  //   reply.error(ENOSYS);
  // }
  // fn flush(
  //   &mut self,
  //   _req: &Request<'_>,
  //   _ino: u64,
  //   _fh: u64,
  //   _lock_owner: u64,
  //   reply: ReplyEmpty,
  // ) {
  //   println!("{}", "flush".red());
  //   reply.error(ENOSYS);
  // }

  // fn release(
  //   &mut self,
  //   _req: &Request<'_>,
  //   _ino: u64,
  //   _fh: u64,
  //   _flags: u32,
  //   _lock_owner: u64,
  //   _flush: bool,
  //   reply: ReplyEmpty,
  // ) {
  //   println!("{}", "release".red());
  //   reply.ok();
  // }
  // fn fsync(&mut self, _req: &Request<'_>, _ino: u64, _fh: u64, _datasync: bool, reply: ReplyEmpty) {
  //   println!("{}", "fsync".red());
  //   reply.error(ENOSYS);
  // }

  // fn opendir(&mut self, _req: &Request<'_>, _ino: u64, _flags: u32, reply: ReplyOpen) {
  //   println!("{} {}", "opendir".blue(), _ino);
  //   reply.opened(69, 0);
  // }

  // fn releasedir(
  //   &mut self,
  //   _req: &Request<'_>,
  //   _ino: u64,
  //   _fh: u64,
  //   _flags: u32,
  //   reply: ReplyEmpty,
  // ) {
  //   println!("{}", "releasedir".red());
  //   reply.ok();
  // }

  // fn fsyncdir(
  //   &mut self,
  //   _req: &Request<'_>,
  //   _ino: u64,
  //   _fh: u64,
  //   _datasync: bool,
  //   reply: ReplyEmpty,
  // ) {
  //   println!("{}", "fsyncdir".red());
  //   reply.error(ENOSYS);
  // }

  // fn statfs(&mut self, _req: &Request<'_>, _ino: u64, reply: ReplyStatfs) {
  //   println!("{} {}", "statfs".red(), _ino);
  //   reply.statfs(0, 0, 0, 0, 0, 512, 255, 0);
  // }

  // fn setxattr(
  //   &mut self,
  //   _req: &Request<'_>,
  //   _ino: u64,
  //   _name: &OsStr,
  //   _value: &[u8],
  //   _flags: u32,
  //   _position: u32,
  //   reply: ReplyEmpty,
  // ) {
  //   println!("{}", "setxattr".red());
  //   reply.error(ENOSYS);
  // }

  // fn removexattr(&mut self, _req: &Request<'_>, _ino: u64, _name: &OsStr, reply: ReplyEmpty) {
  //   println!("{}", "removexattr".red());
  //   reply.error(ENOSYS);
  // }

  // fn access(&mut self, _req: &Request<'_>, _ino: u64, _mask: u32, reply: ReplyEmpty) {
  //   println!("{}", "access".red());
  //   reply.error(ENOSYS);
  // }

  // fn create(
  //   &mut self,
  //   _req: &Request<'_>,
  //   _parent: u64,
  //   _name: &OsStr,
  //   _mode: u32,
  //   _flags: u32,
  //   reply: ReplyCreate,
  // ) {
  //   println!("{}", "create".red());
  //   reply.error(ENOSYS);
  // }

  // fn getlk(
  //   &mut self,
  //   _req: &Request<'_>,
  //   _ino: u64,
  //   _fh: u64,
  //   _lock_owner: u64,
  //   _start: u64,
  //   _end: u64,
  //   _typ: u32,
  //   _pid: u32,
  //   reply: ReplyLock,
  // ) {
  //   println!("{}", "getlk".red());
  //   reply.error(ENOSYS);
  // }

  // fn setlk(
  //   &mut self,
  //   _req: &Request<'_>,
  //   _ino: u64,
  //   _fh: u64,
  //   _lock_owner: u64,
  //   _start: u64,
  //   _end: u64,
  //   _typ: u32,
  //   _pid: u32,
  //   _sleep: bool,
  //   reply: ReplyEmpty,
  // ) {
  //   println!("{}", "setlk".red());
  //   reply.error(ENOSYS);
  // }

  // fn bmap(&mut self, _req: &Request<'_>, _ino: u64, _blocksize: u32, _idx: u64, reply: ReplyBmap) {
  //   println!("{}", "bmap".red());
  //   reply.error(ENOSYS);
  // }

  // #[cfg(target_os = "macos")]
  // fn setvolname(&mut self, _req: &Request<'_>, _name: &OsStr, reply: ReplyEmpty) {
  //   println!("{}", "setvolname".red());
  //   reply.error(ENOSYS);
  // }

  // #[cfg(target_os = "macos")]
  // fn exchange(
  //   &mut self,
  //   _req: &Request<'_>,
  //   _parent: u64,
  //   _name: &OsStr,
  //   _newparent: u64,
  //   _newname: &OsStr,
  //   _options: u64,
  //   reply: ReplyEmpty,
  // ) {
  //   println!("{}", "exchange".red());
  //   reply.error(ENOSYS);
  // }

  // #[cfg(target_os = "macos")]
  // fn getxtimes(&mut self, _req: &Request<'_>, _ino: u64, reply: ReplyXTimes) {
  //   println!("{}", "getxtimes".red());
  //   reply.error(ENOSYS);
  // }
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

  fuse::mount(HelloFS::new(icon_manager)?, mountpoint, &options).unwrap();
  Ok(())
}
