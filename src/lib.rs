#![no_std]

use crate::manager::ObjectManager;

extern crate alloc;

pub mod acpi;
pub mod driver;
pub mod pci;
pub mod manager;
pub mod object;

pub static OBJECT_MANAGER: spin::Once<spin::Mutex<ObjectManager>> =
    spin::Once::new();

pub fn init() {
    OBJECT_MANAGER.call_once(|| spin::Mutex::new(ObjectManager::new()));
}
