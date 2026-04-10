#![no_std]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec::Vec;

use spin::RwLock;

pub type InodeRef = Arc<RwLock<Inode>>;
pub type DentryRef = Arc<RwLock<Dentry>>;
pub type SuperblockRef = Arc<Superblock>;

pub enum InodeKind {
    File(Vec<u8>),
    Directory(BTreeMap<String, DentryRef>),
}

pub struct Inode {
    pub kind: InodeKind,
}

pub struct Dentry {
    pub name: String,
    pub inode: InodeRef,
    pub mounted: Option<SuperblockRef>,
}

pub struct Superblock {
    pub root: DentryRef,
}

impl Superblock {
    pub fn new() -> Self {
        let root_inode = Arc::new(RwLock::new(Inode {
            kind: InodeKind::Directory(BTreeMap::new()),
        }));

        let root = Arc::new(RwLock::new(Dentry {
            name: "/".into(),
            inode: root_inode,
            mounted: None,
        }));

        Self { root }
    }
}

pub struct Vfs {
    pub root: SuperblockRef,
}

impl Vfs {
    pub fn new(root: SuperblockRef) -> Self {
        Self { root }
    }

    fn split_path<'a>(path: &'a str) -> impl Iterator<Item = &'a str> {
        path.split('/').filter(|s| !s.is_empty())
    }

    pub fn resolve(&self, path: &str) -> Option<DentryRef> {
        let mut current = self.root.root.clone();

        for segment in Self::split_path(path) {
            let mounted = { current.read().mounted.clone() };
            if let Some(fs) = mounted {
                current = fs.root.clone();
            }

            let next = {
                let d = current.read();
                let inode = d.inode.read();

                match &inode.kind {
                    InodeKind::Directory(children) => children.get(segment).cloned(),
                    _ => None,
                }
            }?;

            current = next;
        }

        let mounted = { current.read().mounted.clone() };
        if let Some(fs) = mounted {
            return Some(fs.root.clone());
        }

        Some(current)
    }

    pub fn mount(&self, path: &str, fs: SuperblockRef) -> bool {
        let target = match self.resolve(path) {
            Some(t) => t,
            None => return false,
        };

        target.write().mounted = Some(fs);
        true
    }

    pub fn mkdir(&self, path: &str) -> bool {
        let mut current = self.root.root.clone();

        for segment in Self::split_path(path) {
            let next = {
                let d = current.write();
                let mut inode = d.inode.write();

                match &mut inode.kind {
                    InodeKind::Directory(children) => children
                        .entry(segment.to_string())
                        .or_insert_with(|| {
                            Arc::new(RwLock::new(Dentry {
                                name: segment.to_string(),
                                inode: Arc::new(RwLock::new(Inode {
                                    kind: InodeKind::Directory(BTreeMap::new()),
                                })),
                                mounted: None,
                            }))
                        })
                        .clone(),
                    _ => return false,
                }
            };

            current = next;
        }

        true
    }

    pub fn write_file(&self, path: &str, data: Vec<u8>) -> bool {
        let (parent_path, file_name) = match path.rsplit_once('/') {
            Some((parent, name)) if !name.is_empty() => {
                let parent = if parent.is_empty() { "/" } else { parent };
                (parent, name)
            }
            Some(("", name)) if !name.is_empty() => ("/", name),
            _ => return false,
        };

        let parent = match self.resolve(parent_path) {
            Some(p) => p,
            None => return false,
        };

        let d = parent.write();
        let mut inode = d.inode.write();

        match &mut inode.kind {
            InodeKind::Directory(children) => {
                children.insert(
                    file_name.to_string(),
                    Arc::new(RwLock::new(Dentry {
                        name: file_name.to_string(),
                        inode: Arc::new(RwLock::new(Inode {
                            kind: InodeKind::File(data),
                        })),
                        mounted: None,
                    })),
                );
                true
            }
            _ => false,
        }
    }

    pub fn read_file(&self, path: &str) -> Option<Vec<u8>> {
        let dentry = self.resolve(path)?;
        let inode_ref = { dentry.read().inode.clone() };
        let inode = inode_ref.read();

        match &inode.kind {
            InodeKind::File(data) => Some(data.clone()),
            _ => None,
        }
    }
}