extern crate alloc;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::format;
use alloc::string::ToString;
use alloc::vec::Vec;

use spin::Mutex;

use crate::traits::FileSystem;
use crate::types::{Node, NodeKind, Stat};
use crate::errors::{Result, Error};

static MOUNTS: Mutex<Vec<(String, Box<dyn FileSystem>)>> = Mutex::new(Vec::new());

fn normalize_mount_point(m: &str) -> String {
    let mut s = m.trim_end_matches('/').to_string();
    if s.is_empty() {
        s = "/".to_string();
    }
    if !s.starts_with('/') {
        s.insert(0, '/');
    }
    s
}

fn node_to_stat(node: &Node) -> Stat {
    let mut out = Stat::default();
    out.ino = node.inode;
    out.size = node.size as u64;
    out.mode = match &node.kind {
        NodeKind::Dir => 0o040755,
        NodeKind::File => 0o100644,
        NodeKind::Symlink(_) => 0o120777,
        NodeKind::Device => 0o020600,
    };
    out
}

/// Mount a filesystem at `mount_point`. `mount_point` should be absolute.
pub fn mount(mount_point: &str, fs: Box<dyn FileSystem>) -> Result<()> {
    let mp = normalize_mount_point(mount_point);
    fs.mount()?;
    let mut mounts = MOUNTS.lock();
    for (existing, _) in mounts.iter() {
        if *existing == mp {
            return Err(Error::AlreadyExists);
        }
    }
    mounts.push((mp, fs));
    Ok(())
}

/// Open a path and return a file descriptor.
pub fn open(path: &str, flags: u32) -> Result<usize> {
    let mounts = MOUNTS.lock();

    // find longest prefix match
    let mut best_idx: Option<usize> = None;
    let mut best_len: usize = 0;
    for (i, (mp, _fs)) in mounts.iter().enumerate() {
        if mp == "/" {
            if path.starts_with('/') && 1 > best_len {
                best_idx = Some(i);
                best_len = 1;
            }
        } else if path == mp || path.starts_with(&format!("{}/", mp)) {
            let l = mp.len();
            if l > best_len {
                best_len = l;
                best_idx = Some(i);
            }
        }
    }

    let idx = best_idx.ok_or(Error::NotFound)?;
    let (mp, fs) = &mounts[idx];

    let local_path: &str = if mp == "/" { path } else if path == mp.as_str() { "/" } else { &path[mp.len()..] };

    let fileobj = fs.open(local_path, flags)?;
    // allocate fd (this calls into fd table which uses its own lock)
    let fd = crate::fd::alloc_fd(fileobj);
    Ok(fd)
}

pub fn read(fd: usize, buf: &mut [u8]) -> Result<usize> {
    crate::fd::read(fd, buf)
}

pub fn write(fd: usize, buf: &[u8]) -> Result<usize> {
    crate::fd::write(fd, buf)
}

pub fn close(fd: usize) -> Result<()> {
    crate::fd::close(fd)
}

fn lookup_node(path: &str) -> Result<Node> {
    let mounts = MOUNTS.lock();

    // find longest prefix match
    let mut best_idx: Option<usize> = None;
    let mut best_len: usize = 0;
    for (i, (mp, _fs)) in mounts.iter().enumerate() {
        if mp == "/" {
            if path.starts_with('/') && 1 > best_len {
                best_idx = Some(i);
                best_len = 1;
            }
        } else if path == mp || path.starts_with(&format!("{}/", mp)) {
            let l = mp.len();
            if l > best_len {
                best_len = l;
                best_idx = Some(i);
            }
        }
    }

    let idx = best_idx.ok_or(Error::NotFound)?;
    let (mp, fs) = &mounts[idx];

    let local_path: &str = if mp == "/" { path } else if path == mp.as_str() { "/" } else { &path[mp.len()..] };

    fs.lookup(local_path)
}

/// lstat: return metadata for the path without following symlinks
pub fn lstat(path: &str) -> Result<Stat> {
    let node = lookup_node(path)?;
    Ok(node_to_stat(&node))
}

/// stat: like lstat, but follow symlinks up to a reasonable depth
pub fn stat(path: &str) -> Result<Stat> {
    stat_follow(path, 0)
}

fn stat_follow(path: &str, depth: usize) -> Result<Stat> {
    if depth > 16 {
        return Err(Error::InvalidInput);
    }
    let node = lookup_node(path)?;
    match node.kind {
        NodeKind::Symlink(target) => {
            let new_path = if target.starts_with('/') {
                target
            } else {
                // compute parent of `path`
                let parent = if path == "/" {
                    "/".to_string()
                } else {
                    match path.rfind('/') {
                        Some(0) => "/".to_string(),
                        Some(p) => path[..p].to_string(),
                        None => "/".to_string(),
                    }
                };
                if parent == "/" {
                    format!("/{}", target)
                } else {
                    format!("{}/{}", parent, target)
                }
            };
            stat_follow(&new_path, depth + 1)
        }
        _ => Ok(node_to_stat(&node)),
    }
}

/// fstat: stat a file descriptor
pub fn fstat(fd: usize) -> Result<Stat> {
    crate::fd::fstat(fd)
}

pub fn init() -> Result<()> {
    // no-op: mounts are empty by default; user/kernel may mount filesystems later
    Ok(())
}
