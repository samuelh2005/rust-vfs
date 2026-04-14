#![no_std]

use crate::{driver::init_drivers, manager::ObjectManager};

extern crate alloc;

pub mod acpi;
pub mod driver;
pub mod manager;
pub mod object;
pub mod pci;

pub static OBJECT_MANAGER: spin::Once<spin::Mutex<ObjectManager>> = spin::Once::new();

pub fn init() {
    OBJECT_MANAGER.call_once(|| spin::Mutex::new(ObjectManager::new()));
    init_drivers();
}
