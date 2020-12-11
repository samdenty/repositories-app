use crate::client;
use crate::icon_manager::IconManager;
use crate::Organization;
use fuse::*;
use github_rs::client::Executor;
use libc::ENOENT;
use libc::ENOSYS;
use std::io::Error;
use std::time::SystemTime;
use std::time::{Duration, UNIX_EPOCH};
use std::{convert::TryInto, path::Path};
use std::{env, ffi::OsStr};

const TTL: Duration = Duration::from_secs(1); // 1 second

const HELLO_DIR_ATTR: FileAttr = FileAttr {
  ino: 1,
  size: 0,
  blocks: 0,
  atime: UNIX_EPOCH, // 1970-01-01 00:00:00
  mtime: UNIX_EPOCH,
  ctime: UNIX_EPOCH,
  crtime: UNIX_EPOCH,
  kind: FileType::Directory,
  perm: 0o755,
  nlink: 2,
  uid: 501,
  gid: 20,
  rdev: 0,
  flags: 0,
};

const HELLO_TXT_CONTENT: &str = "Hello World!\n";

const HELLO_TXT_ATTR: FileAttr = FileAttr {
  ino: 2,
  size: 13,
  blocks: 1,
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

const ICON_TXT_ATTR: FileAttr = FileAttr {
  ino: 3,
  size: 13,
  blocks: 1,
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

impl Filesystem for HelloFS {
  fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
    if name == "Icon\r" {
      reply.entry(
        &TTL,
        &FileAttr {
          ino: 10,
          size: 0,
          blocks: 1,
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
        },
        0,
      );
      return;
    }

    if parent == 1 {
      let index = self
        .orgs
        .iter()
        .position(|org| org.login == name.to_str().unwrap());

      if let Some(index) = index {
        println!("lookup {:?}", name.to_str());
        reply.entry(
          &TTL,
          &FileAttr {
            ino: (index + 2).try_into().unwrap(),
            size: 0,
            blocks: 0,
            atime: UNIX_EPOCH, // 1970-01-01 00:00:00
            mtime: UNIX_EPOCH,
            ctime: UNIX_EPOCH,
            crtime: UNIX_EPOCH,
            kind: FileType::Directory,
            perm: 0o755,
            nlink: 1,
            uid: 501,
            gid: 20,
            rdev: 0,
            flags: 0,
          },
          0,
        );
        return;
      }
    }

    println!("enoent lookup {:?} {}", name.to_str(), parent);

    reply.error(ENOENT);
  }

  fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
    let org = if ino >= 2 {
      self.orgs.get((ino - 2) as usize)
    } else {
      None
    };

    if ino == 1 {
      println!("getattr: {}", ino);
      reply.attr(
        &TTL,
        &FileAttr {
          ino: 1,
          size: 0,
          blocks: 0,
          atime: UNIX_EPOCH, // 1970-01-01 00:00:00
          mtime: UNIX_EPOCH,
          ctime: UNIX_EPOCH,
          crtime: UNIX_EPOCH,
          kind: FileType::Directory,
          perm: 0o755,
          nlink: 1,
          uid: 501,
          gid: 20,
          rdev: 0,
          flags: 0,
        },
      )
    } else if ino == 10 {
      println!("getattr: {}", ino);
      reply.attr(
        &TTL,
        &FileAttr {
          ino: 10,
          size: 13,
          blocks: 1,
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
        },
      )
    } else if let Some(org) = org {
      println!("getattr: {}", ino);
      reply.attr(
        &TTL,
        &FileAttr {
          ino,
          size: 0,
          blocks: 0,
          atime: UNIX_EPOCH, // 1970-01-01 00:00:00
          mtime: UNIX_EPOCH,
          ctime: UNIX_EPOCH,
          crtime: UNIX_EPOCH,
          kind: FileType::Directory,
          perm: 0o755,
          nlink: 1,
          uid: 501,
          gid: 20,
          rdev: 0,
          flags: 0,
        },
      )
    } else {
      println!("enoent getattr: {}", ino);
      reply.error(ENOENT)
    }
  }

  fn read(
    &mut self,
    _req: &Request,
    ino: u64,
    _fh: u64,
    offset: i64,
    _size: u32,
    reply: ReplyData,
  ) {
    println!("read {}", ino);

    if ino == 10 {
      reply.data(&HELLO_TXT_CONTENT.as_bytes()[offset as usize..]);
    } else {
      reply.error(ENOENT);
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
    println!("readdir {}", ino);
    if ino == 1 {
      let mut entries = vec![
        (1, FileType::Directory, "."),
        (1, FileType::Directory, ".."),
      ];
      for (i, org) in self.orgs.iter().enumerate() {
        entries.push(((i + 2) as u64, FileType::Directory, org.login.as_str()));
        // println!("{:#?}", org.login);
      }
      for (i, entry) in entries.into_iter().enumerate().skip(offset as usize) {
        // i + 1 means the index of the next entry
        reply.add(entry.0, (i + 1) as i64, entry.1, entry.2);
      }
      reply.ok();
    } else if ino == 4 {
      let entries = vec![
        (4, FileType::Directory, "."),
        (4, FileType::Directory, ".."),
        (10, FileType::RegularFile, "Icon\r"),
      ];
      for (i, entry) in entries.into_iter().enumerate().skip(offset as usize) {
        // i + 1 means the index of the next entry
        reply.add(entry.0, (i + 1) as i64, entry.1, entry.2);
      }
      reply.ok();
    } else {
      reply.error(ENOENT);
    }
  }

  // fn destroy(&mut self, _req: &Request<'_>) {
  //   println!("destroy");
  // }

  // fn forget(&mut self, _req: &Request<'_>, _ino: u64, _nlookup: u64) {
  //   println!("forget");
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
  //   println!("setattr");
  //   reply.error(ENOSYS);
  // }

  // fn readlink(&mut self, _req: &Request<'_>, _ino: u64, reply: ReplyData) {
  //   println!("readlink");
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
  //   println!("mknod");
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
  //   println!("mkdir");
  //   reply.entry(&TTL, &HELLO_TXT_ATTR, 0)
  //   // reply.error(ENOSYS);
  // }

  // fn unlink(&mut self, _req: &Request<'_>, _parent: u64, _name: &OsStr, reply: ReplyEmpty) {
  //   println!("unlink");
  //   reply.error(ENOSYS);
  // }

  // fn rmdir(&mut self, _req: &Request<'_>, _parent: u64, _name: &OsStr, reply: ReplyEmpty) {
  //   println!("rmdir");
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
  //   println!("symlink");
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
  //   println!("rename");
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
  //   println!("link");
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
  //   println!("write");
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
  //   println!("flush");
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
  //   // println!("release");
  //   reply.ok();
  // }
  // fn fsync(&mut self, _req: &Request<'_>, _ino: u64, _fh: u64, _datasync: bool, reply: ReplyEmpty) {
  //   println!("fsync");
  //   reply.error(ENOSYS);
  // }

  // fn opendir(&mut self, _req: &Request<'_>, _ino: u64, _flags: u32, reply: ReplyOpen) {
  //   // println!("opendir");
  //   reply.opened(0, 0);
  // }

  // fn releasedir(
  //   &mut self,
  //   _req: &Request<'_>,
  //   _ino: u64,
  //   _fh: u64,
  //   _flags: u32,
  //   reply: ReplyEmpty,
  // ) {
  //   // println!("releasedir");
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
  //   println!("fsyncdir");
  //   reply.error(ENOSYS);
  // }

  // fn statfs(&mut self, _req: &Request<'_>, _ino: u64, reply: ReplyStatfs) {
  //   // println!("statfs");
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
  //   println!("setxattr");
  //   reply.error(ENOSYS);
  // }

  // fn getxattr(
  //   &mut self,
  //   _req: &Request<'_>,
  //   _ino: u64,
  //   _name: &OsStr,
  //   _size: u32,
  //   reply: ReplyXattr,
  // ) {
  //   println!("getxattr");
  //   reply.
  //   reply.error(ENOSYS);
  // }

  // fn listxattr(&mut self, _req: &Request<'_>, _ino: u64, _size: u32, reply: ReplyXattr) {
  //   println!("listxattr");
  //   reply.data();
  //   reply.error(ENOSYS);
  // }

  // fn removexattr(&mut self, _req: &Request<'_>, _ino: u64, _name: &OsStr, reply: ReplyEmpty) {
  //   println!("removexattr");
  //   reply.error(ENOSYS);
  // }

  // fn access(&mut self, _req: &Request<'_>, _ino: u64, _mask: u32, reply: ReplyEmpty) {
  //   println!("access");
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
  //   println!("create");
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
  //   println!("getlk");
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
  //   println!("setlk");
  //   reply.error(ENOSYS);
  // }

  // fn bmap(&mut self, _req: &Request<'_>, _ino: u64, _blocksize: u32, _idx: u64, reply: ReplyBmap) {
  //   println!("bmap");
  //   reply.error(ENOSYS);
  // }

  // #[cfg(target_os = "macos")]
  // fn setvolname(&mut self, _req: &Request<'_>, _name: &OsStr, reply: ReplyEmpty) {
  //   println!("setvolname");
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
  //   println!("exchange");
  //   reply.error(ENOSYS);
  // }

  // #[cfg(target_os = "macos")]
  // fn getxtimes(&mut self, _req: &Request<'_>, _ino: u64, reply: ReplyXTimes) {
  //   println!("getxtimes");
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
    "ovolicon=/Users/samdenty/Downloads/25231.icns",
  ]
  .iter()
  .map(|o| o.as_ref())
  .collect::<Vec<&OsStr>>();

  let mountpoint = env::args_os().nth(1).unwrap();

  fuse::mount(HelloFS::new(icon_manager)?, mountpoint, &options).unwrap();
  Ok(())
}
