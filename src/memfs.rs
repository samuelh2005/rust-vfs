extern crate alloc;

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::format;
use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::sync::Arc;

use core::sync::atomic::{AtomicU64, Ordering};

use spin::Mutex;

use crate::traits::{FileSystem, FileObject};
use crate::types::{Node, NodeKind, Stat};
use crate::errors::{Result, Error};

/// MemFS stores nodes in a flat map keyed by absolute path (e.g. "/foo/bar").
/// This avoids performing component-by-component traversal inside the FS;
/// callers must provide absolute paths.
#[derive(Debug)]
enum MemNodeKind {
    File { content: Mutex<Vec<u8>> },
    Dir,
    Symlink { target: String },
}

#[derive(Debug)]
struct MemNode {
    kind: MemNodeKind,
    inode: u64,
}

pub struct MemFS {
    map: Mutex<BTreeMap<String, Arc<MemNode>>>,
    ino: AtomicU64,
}

impl MemFS {
    pub fn new() -> Self {
        let mut map = BTreeMap::new();
        // root directory
        map.insert("/".into(), Arc::new(MemNode { kind: MemNodeKind::Dir, inode: 1 }));
        MemFS { map: Mutex::new(map), ino: AtomicU64::new(2) }
    }

    fn normalize(path: &str) -> Result<String> {
        if path.is_empty() {
            return Err(Error::InvalidInput);
        }
        if !path.starts_with('/') {
            return Err(Error::InvalidInput);
        }
        // Collapse repeated slashes and remove trailing slash (except root)
        let mut out = String::new();
        let mut was_slash = false;
        for c in path.chars() {
            if c == '/' {
                if was_slash { continue; }
                was_slash = true;
                out.push('/');
            } else {
                was_slash = false;
                out.push(c);
            }
        }
        if out.len() > 1 && out.ends_with('/') {
            out.pop();
        }
        Ok(out)
    }

    fn allocate_inode(&self) -> u64 {
        self.ino.fetch_add(1, Ordering::SeqCst)
    }

    fn parent_path(path: &str) -> &str {
        if path == "/" { return "/"; }
        match path.rfind('/') {
            Some(0) => "/",
            Some(i) => &path[..i],
            None => "/",
        }
    }

    pub fn create_dir_all(&self, path: &str) -> Result<()> {
        let p = Self::normalize(path)?;
        if p == "/" { return Ok(()); }
        let mut accum = String::new();
        for comp in p.split('/') {
            if comp.is_empty() { accum = "/".to_string(); continue; }
            if accum == "/" { accum = format!("/{}", comp); } else { accum = format!("{}/{}", accum, comp); }
            let mut map = self.map.lock();
            if !map.contains_key(&accum) {
                let ino = self.allocate_inode();
                map.insert(accum.clone(), Arc::new(MemNode { kind: MemNodeKind::Dir, inode: ino }));
            }
        }
        Ok(())
    }

    pub fn symlink(&self, target: &str, linkpath: &str) -> Result<()> {
        let lp = Self::normalize(linkpath)?;
        let parent = Self::parent_path(&lp);
        let map = &mut *self.map.lock();
        if !map.contains_key(parent) {
            return Err(Error::NotFound);
        }
        if map.contains_key(&lp) {
            return Err(Error::AlreadyExists);
        }
        let ino = self.allocate_inode();
        map.insert(lp, Arc::new(MemNode { kind: MemNodeKind::Symlink { target: target.to_string() }, inode: ino }));
        Ok(())
    }
}

struct MemFsFile {
    path: String,
    node: Arc<MemNode>,
    pos: usize,
}

impl MemFsFile {
    fn new(path: String, node: Arc<MemNode>) -> Self { MemFsFile { path, node, pos: 0 } }
}

impl FileObject for MemFsFile {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match &self.node.kind {
            MemNodeKind::File { content } => {
                let c = content.lock();
                if self.pos >= c.len() { return Ok(0); }
                let n = core::cmp::min(buf.len(), c.len() - self.pos);
                buf[..n].copy_from_slice(&c[self.pos..self.pos + n]);
                self.pos += n;
                Ok(n)
            }
            _ => Err(Error::IsDir),
        }
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        match &self.node.kind {
            MemNodeKind::File { content } => {
                let mut c = content.lock();
                c.extend_from_slice(buf);
                Ok(buf.len())
            }
            _ => Err(Error::IsDir),
        }
    }

    fn stat(&self) -> Result<Stat> {
        let mut out = Stat::default();
        out.ino = self.node.inode;
        out.size = match &self.node.kind {
            MemNodeKind::File { content } => content.lock().len() as u64,
            _ => 0,
        };
        Ok(out)
    }

    fn close(&mut self) -> Result<()> { Ok(()) }
}

impl FileSystem for MemFS {
    fn mount(&self) -> Result<()> { Ok(()) }

    fn lookup(&self, path: &str) -> Result<Node> {
        let p = Self::normalize(path)?;
        let map = self.map.lock();
        if let Some(node) = map.get(&p) {
            let kind = match &node.kind {
                MemNodeKind::Dir => NodeKind::Dir,
                MemNodeKind::File { .. } => NodeKind::File,
                MemNodeKind::Symlink { target } => NodeKind::Symlink(target.clone()),
            };
            let size = match &node.kind {
                MemNodeKind::File { content } => content.lock().len(),
                _ => 0,
            };
            // name is last component
            let name = if p == "/" { "/".into() } else { p.rsplit('/').next().unwrap_or("").to_string() };
            return Ok(Node { name, kind, size, inode: node.inode });
        }
        Err(Error::NotFound)
    }

    fn open(&self, path: &str, _flags: u32) -> Result<Box<dyn FileObject>> {
        let p = Self::normalize(path)?;
        let mut map = self.map.lock();
        if let Some(node) = map.get(&p) {
            match &node.kind {
                MemNodeKind::File { .. } => return Ok(Box::new(MemFsFile::new(p.clone(), node.clone()))),
                MemNodeKind::Dir => return Err(Error::IsDir),
                MemNodeKind::Symlink { .. } => return Err(Error::NotSupported),
            }
        }
        // create parent if necessary
        let parent = Self::parent_path(&p).to_string();
        if !map.contains_key(&parent) {
            drop(map);
            self.create_dir_all(&parent)?;
            map = self.map.lock();
        }
        // create file
        if map.contains_key(&p) {
            let node = map.get(&p).unwrap().clone();
            match &node.kind {
                MemNodeKind::File { .. } => return Ok(Box::new(MemFsFile::new(p.clone(), node.clone()))),
                _ => return Err(Error::InvalidInput),
            }
        }
        let ino = self.allocate_inode();
        let node = Arc::new(MemNode { kind: MemNodeKind::File { content: Mutex::new(Vec::new()) }, inode: ino });
        map.insert(p.clone(), node.clone());
        Ok(Box::new(MemFsFile::new(p, node)))
    }
}
