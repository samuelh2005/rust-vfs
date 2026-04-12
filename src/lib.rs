#![no_std]

extern crate alloc;

pub mod core;
pub mod traits;
pub mod types;
pub mod errors;
pub mod memfs;
pub mod fd;

pub use core::{init, mount, open, read, write, close, stat, fstat, lstat};
pub use traits::FileSystem;
pub use types::{Node, NodeKind, Stat};
pub use errors::{Error, Result};
