use spin::{Mutex, Once};
use alloc::{boxed::Box, vec::Vec};

use crate::pci::{PCIDeviceHeader, PCIHeaderType0};

pub trait PciDriver: Send {
    fn supports(&self, vendor_id: u16, device_id: u16, class: u8, subclass: u8) -> bool;

    fn init(&self, pci: &PCIDeviceHeader, func: &PCIHeaderType0);
}

pub static DRIVERS: Once<Mutex<Vec<Box<dyn PciDriver>>>> = Once::new();

pub fn probe_drivers(pci_header: &PCIDeviceHeader, pci_function: &PCIHeaderType0) {
    let drivers = DRIVERS.get().expect("PCI drivers not initialized");
    let drivers = drivers.lock();

    for driver in drivers.iter() {
        if driver.supports(
            pci_header.vendor_id,
            pci_header.device_id,
            pci_header.class_code,
            pci_header.class_code,
        ) {
            driver.init(pci_header, pci_function);
        }
    }
}

pub fn init_drivers() {
    let drivers = Vec::new();
    DRIVERS.call_once(|| Mutex::new(drivers));
}

pub fn register_driver(driver: Box<dyn PciDriver>) {
    let drivers = DRIVERS.get().expect("PCI drivers not initialized");
    drivers.lock().push(driver);
}