use fuse::ReplyBmap;
use fuse::ReplyCreate;
use fuse::ReplyEmpty;
use fuse::ReplyLock;
use fuse::ReplyOpen;
use fuse::ReplyStatfs;
use fuse::ReplyXTimes;
use fuse::ReplyXattr;
use fuse::{
  FileAttr, FileType, Filesystem, ReplyAttr, ReplyData, ReplyDirectory, ReplyEntry, ReplyWrite,
  Request,
};
use libc::ENOENT;
use libc::ENOSYS;
use std::env;
use std::ffi::OsStr;
use std::path::Path;
use std::time::SystemTime;
use std::time::{Duration, UNIX_EPOCH};

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

struct HelloFS;

impl Filesystem for HelloFS {
  fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
    println!("lookup {}", name.to_str());
    if parent == 1 {
      if name.to_str() == Some("hello.txt") {
        reply.entry(&TTL, &HELLO_TXT_ATTR, 0);
      } else {
        reply.entry(&TTL, &ICON_TXT_ATTR, 0);
      }
    } else {
      reply.error(ENOENT);
    }
  }

  fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
    // println!("getattr");
    match ino {
      1 => reply.attr(&TTL, &HELLO_DIR_ATTR),
      2 => reply.attr(&TTL, &HELLO_TXT_ATTR),
      3 => reply.attr(&TTL, &ICON_TXT_ATTR),
      _ => reply.error(ENOENT),
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

    if ino == 3 {
      reply.data(&"test".as_bytes()[offset as usize..])
    } else if ino == 2 {
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
    // println!("readdir");
    if ino != 1 {
      reply.error(ENOENT);
      return;
    }

    let entries = vec![
      (1, FileType::Directory, "."),
      (1, FileType::Directory, ".."),
      (2, FileType::RegularFile, "hello.txt"),
      (3, FileType::RegularFile, "Icon\r"),
    ];

    for (i, entry) in entries.into_iter().enumerate().skip(offset as usize) {
      // i + 1 means the index of the next entry
      reply.add(entry.0, (i + 1) as i64, entry.1, entry.2);
    }
    reply.ok();
  }

  fn destroy(&mut self, _req: &Request<'_>) {
    println!("destroy");
  }

  fn forget(&mut self, _req: &Request<'_>, _ino: u64, _nlookup: u64) {
    println!("forget");
  }

  fn setattr(
    &mut self,
    _req: &Request<'_>,
    _ino: u64,
    _mode: Option<u32>,
    _uid: Option<u32>,
    _gid: Option<u32>,
    _size: Option<u64>,
    _atime: Option<SystemTime>,
    _mtime: Option<SystemTime>,
    _fh: Option<u64>,
    _crtime: Option<SystemTime>,
    _chgtime: Option<SystemTime>,
    _bkuptime: Option<SystemTime>,
    _flags: Option<u32>,
    reply: ReplyAttr,
  ) {
    println!("setattr");
    reply.error(ENOSYS);
  }

  fn readlink(&mut self, _req: &Request<'_>, _ino: u64, reply: ReplyData) {
    println!("readlink");
    reply.error(ENOSYS);
  }

  fn mknod(
    &mut self,
    _req: &Request<'_>,
    _parent: u64,
    _name: &OsStr,
    _mode: u32,
    _rdev: u32,
    reply: ReplyEntry,
  ) {
    println!("mknod");
    reply.error(ENOSYS);
  }

  fn mkdir(
    &mut self,
    _req: &Request<'_>,
    _parent: u64,
    _name: &OsStr,
    _mode: u32,
    reply: ReplyEntry,
  ) {
    println!("mkdir");
    reply.entry(&TTL, &HELLO_TXT_ATTR, 0)
    // reply.error(ENOSYS);
  }

  fn unlink(&mut self, _req: &Request<'_>, _parent: u64, _name: &OsStr, reply: ReplyEmpty) {
    println!("unlink");
    reply.error(ENOSYS);
  }

  fn rmdir(&mut self, _req: &Request<'_>, _parent: u64, _name: &OsStr, reply: ReplyEmpty) {
    println!("rmdir");
    reply.error(ENOSYS);
  }

  fn symlink(
    &mut self,
    _req: &Request<'_>,
    _parent: u64,
    _name: &OsStr,
    _link: &Path,
    reply: ReplyEntry,
  ) {
    println!("symlink");
    reply.error(ENOSYS);
  }

  fn rename(
    &mut self,
    _req: &Request<'_>,
    _parent: u64,
    _name: &OsStr,
    _newparent: u64,
    _newname: &OsStr,
    reply: ReplyEmpty,
  ) {
    println!("rename");
    reply.error(ENOSYS);
  }

  fn link(
    &mut self,
    _req: &Request<'_>,
    _ino: u64,
    _newparent: u64,
    _newname: &OsStr,
    reply: ReplyEntry,
  ) {
    println!("link");
    reply.error(ENOSYS);
  }

  fn open(&mut self, _req: &Request<'_>, _ino: u64, _flags: u32, reply: ReplyOpen) {
    println!("open");
    reply.opened(0, 0);
  }

  fn write(
    &mut self,
    _req: &Request<'_>,
    _ino: u64,
    _fh: u64,
    _offset: i64,
    _data: &[u8],
    _flags: u32,
    reply: ReplyWrite,
  ) {
    println!("write");
    reply.error(ENOSYS);
  }
  fn flush(
    &mut self,
    _req: &Request<'_>,
    _ino: u64,
    _fh: u64,
    _lock_owner: u64,
    reply: ReplyEmpty,
  ) {
    println!("flush");
    reply.error(ENOSYS);
  }

  fn release(
    &mut self,
    _req: &Request<'_>,
    _ino: u64,
    _fh: u64,
    _flags: u32,
    _lock_owner: u64,
    _flush: bool,
    reply: ReplyEmpty,
  ) {
    // println!("release");
    reply.ok();
  }
  fn fsync(&mut self, _req: &Request<'_>, _ino: u64, _fh: u64, _datasync: bool, reply: ReplyEmpty) {
    println!("fsync");
    reply.error(ENOSYS);
  }

  fn opendir(&mut self, _req: &Request<'_>, _ino: u64, _flags: u32, reply: ReplyOpen) {
    // println!("opendir");
    reply.opened(0, 0);
  }

  fn releasedir(
    &mut self,
    _req: &Request<'_>,
    _ino: u64,
    _fh: u64,
    _flags: u32,
    reply: ReplyEmpty,
  ) {
    // println!("releasedir");
    reply.ok();
  }

  fn fsyncdir(
    &mut self,
    _req: &Request<'_>,
    _ino: u64,
    _fh: u64,
    _datasync: bool,
    reply: ReplyEmpty,
  ) {
    println!("fsyncdir");
    reply.error(ENOSYS);
  }

  fn statfs(&mut self, _req: &Request<'_>, _ino: u64, reply: ReplyStatfs) {
    // println!("statfs");
    reply.statfs(0, 0, 0, 0, 0, 512, 255, 0);
  }

  fn setxattr(
    &mut self,
    _req: &Request<'_>,
    _ino: u64,
    _name: &OsStr,
    _value: &[u8],
    _flags: u32,
    _position: u32,
    reply: ReplyEmpty,
  ) {
    println!("setxattr");
    reply.error(ENOSYS);
  }

  fn getxattr(
    &mut self,
    _req: &Request<'_>,
    _ino: u64,
    _name: &OsStr,
    _size: u32,
    reply: ReplyXattr,
  ) {
    println!("getxattr");
    reply.error(ENOSYS);
  }

  fn listxattr(&mut self, _req: &Request<'_>, _ino: u64, _size: u32, reply: ReplyXattr) {
    println!("listxattr");
    reply.error(ENOSYS);
  }

  fn removexattr(&mut self, _req: &Request<'_>, _ino: u64, _name: &OsStr, reply: ReplyEmpty) {
    println!("removexattr");
    reply.error(ENOSYS);
  }

  fn access(&mut self, _req: &Request<'_>, _ino: u64, _mask: u32, reply: ReplyEmpty) {
    println!("access");
    reply.error(ENOSYS);
  }

  fn create(
    &mut self,
    _req: &Request<'_>,
    _parent: u64,
    _name: &OsStr,
    _mode: u32,
    _flags: u32,
    reply: ReplyCreate,
  ) {
    println!("create");
    reply.error(ENOSYS);
  }

  fn getlk(
    &mut self,
    _req: &Request<'_>,
    _ino: u64,
    _fh: u64,
    _lock_owner: u64,
    _start: u64,
    _end: u64,
    _typ: u32,
    _pid: u32,
    reply: ReplyLock,
  ) {
    println!("getlk");
    reply.error(ENOSYS);
  }

  fn setlk(
    &mut self,
    _req: &Request<'_>,
    _ino: u64,
    _fh: u64,
    _lock_owner: u64,
    _start: u64,
    _end: u64,
    _typ: u32,
    _pid: u32,
    _sleep: bool,
    reply: ReplyEmpty,
  ) {
    println!("setlk");
    reply.error(ENOSYS);
  }

  fn bmap(&mut self, _req: &Request<'_>, _ino: u64, _blocksize: u32, _idx: u64, reply: ReplyBmap) {
    println!("bmap");
    reply.error(ENOSYS);
  }

  #[cfg(target_os = "macos")]
  fn setvolname(&mut self, _req: &Request<'_>, _name: &OsStr, reply: ReplyEmpty) {
    println!("setvolname");
    reply.error(ENOSYS);
  }

  #[cfg(target_os = "macos")]
  fn exchange(
    &mut self,
    _req: &Request<'_>,
    _parent: u64,
    _name: &OsStr,
    _newparent: u64,
    _newname: &OsStr,
    _options: u64,
    reply: ReplyEmpty,
  ) {
    println!("exchange");
    reply.error(ENOSYS);
  }

  #[cfg(target_os = "macos")]
  fn getxtimes(&mut self, _req: &Request<'_>, _ino: u64, reply: ReplyXTimes) {
    println!("getxtimes");
    reply.error(ENOSYS);
  }
}

fn main() {
  let mountpoint = env::args_os().nth(1).unwrap();
  println!("{:?}", mountpoint);
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
  fuse::mount(HelloFS, mountpoint, &options).unwrap();
}
