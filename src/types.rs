extern crate alloc;

use alloc::string::String;

#[derive(Clone, Debug)]
pub enum NodeKind {
    File,
    Dir,
    Symlink(String),
    Device,
}

#[derive(Clone, Debug)]
pub struct Node {
    pub name: String,
    pub kind: NodeKind,
    pub size: usize,
    pub inode: u64,
}

#[derive(Clone, Debug, Default)]
pub struct Stat {
    pub ino: u64,
    pub size: u64,
    pub mode: u32,
    pub nlink: u32,
    pub uid: u32,
    pub gid: u32,
    pub atime: u64,
    pub mtime: u64,
    pub ctime: u64,
}
