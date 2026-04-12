extern crate alloc;

use alloc::boxed::Box;
// String is not required here

use crate::types::{Node, Stat};
use crate::errors::Result;

/// Minimal file object trait used by the descriptor layer.
pub trait FileObject: Send {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn stat(&self) -> Result<Stat>;
    fn close(&mut self) -> Result<()>;
}

/// Filesystem trait implemented by concrete filesystem backends.
pub trait FileSystem: Send + Sync {
    /// Called when filesystem is mounted. Implementations may perform init here.
    fn mount(&self) -> Result<()>;

    /// Lookup metadata for a path. Path is relative to the mount point.
    fn lookup(&self, path: &str) -> Result<Node>;

    /// Open a path and return a boxed file object.
    fn open(&self, path: &str, flags: u32) -> Result<Box<dyn FileObject>>;

    /// Create directories as needed for the path.
    fn create_dir_all(&self, path: &str) -> Result<()> {
        let _ = path;
        Err(crate::errors::Error::NotSupported)
    }

    /// Create a symlink.
    fn symlink(&self, _target: &str, _linkpath: &str) -> Result<()> {
        Err(crate::errors::Error::NotSupported)
    }
}

/// Simple trait describing devices used by devfs implementations.
pub trait Device: Send {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn name(&self) -> &str;
}
