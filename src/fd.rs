extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;

use spin::Mutex;

use crate::traits::FileObject;
use crate::errors::{Result, Error};

static FD_TABLE: Mutex<Vec<Option<Box<dyn FileObject>>>> = Mutex::new(Vec::new());

pub type FD = usize;

pub fn alloc_fd(f: Box<dyn FileObject>) -> FD {
    let mut table = FD_TABLE.lock();

    // find an empty slot
    for (i, slot) in table.iter_mut().enumerate() {
        if slot.is_none() {
            *slot = Some(f);
            return i;
        }
    }

    table.push(Some(f));
    table.len() - 1
}

pub fn read(fd: FD, buf: &mut [u8]) -> Result<usize> {
    let mut table = FD_TABLE.lock();
    if fd >= table.len() {
        return Err(Error::BadDescriptor);
    }
    match table[fd].as_mut() {
        Some(obj) => obj.read(buf),
        None => Err(Error::BadDescriptor),
    }
}

pub fn write(fd: FD, buf: &[u8]) -> Result<usize> {
    let mut table = FD_TABLE.lock();
    if fd >= table.len() {
        return Err(Error::BadDescriptor);
    }
    match table[fd].as_mut() {
        Some(obj) => obj.write(buf),
        None => Err(Error::BadDescriptor),
    }
}

pub fn close(fd: FD) -> Result<()> {
    let mut table = FD_TABLE.lock();
    if fd >= table.len() {
        return Err(Error::BadDescriptor);
    }
    if let Some(slot) = table.get_mut(fd) {
        if let Some(mut obj) = slot.take() {
            obj.close()?;
            return Ok(());
        }
    }
    Err(Error::BadDescriptor)
}

pub fn fstat(fd: FD) -> Result<crate::types::Stat> {
    let table = FD_TABLE.lock();
    if fd >= table.len() {
        return Err(Error::BadDescriptor);
    }
    match &table[fd] {
        Some(obj) => obj.stat(),
        None => Err(Error::BadDescriptor),
    }
}
