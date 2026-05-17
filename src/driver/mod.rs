use alloc::{boxed::Box, vec::Vec};
use log::{debug, info};
use spin::{Mutex, Once};

pub mod responses;

use crate::OBJECT_MANAGER;
use crate::driver::responses::DriverResponse;
use crate::pci::{PCIDeviceHeader, PCIHeaderType0};

pub trait PciDriver: Send {
    fn supports(&self, vendor_id: u16, device_id: u16, class: u8, subclass: u8) -> bool;

    /// Called when a matching device is found. Return an `OperationHandler`
    /// to expose the device via the VFS object manager, or `None` if the
    /// driver wants to handle the device without exposing an object.
    fn init(&self, pci: &PCIDeviceHeader, func: &PCIHeaderType0) -> Result<DriverResponse, ()>;
}

pub static DRIVERS: Once<Mutex<Vec<Box<dyn PciDriver>>>> = Once::new();

pub fn probe_drivers(pci_header: &PCIDeviceHeader, pci_function: &PCIHeaderType0) {
    let drivers = DRIVERS.get().expect("PCI drivers not initialized");
    let drivers = drivers.lock();

    // Read fields from the packed PCI header into locals to avoid creating
    // references to fields of a packed struct (which would be potentially
    // unaligned and UB).
    let vendor = pci_header.vendor_id;
    let device = pci_header.device_id;
    let class = pci_header.class_code;
    let subclass = pci_header.subclass;

    let manager_mutex = OBJECT_MANAGER
        .get()
        .expect("Object manager not initialized");
    let mut manager = manager_mutex.lock();

    for driver in drivers.iter() {
        if driver.supports(vendor, device, class, subclass) {
            debug!(
                "Probing PCI device {:04x}:{:04x} (class: {:02x}, subclass: {:02x})",
                vendor, device, class, subclass
            );

            let response = driver.init(pci_header, pci_function);
            if let Ok(response) = response {
                let name = response.object_id;
                info!(
                    "Initializing object {} for PCI device {:04x}:{:04x}",
                    name, vendor, device
                );
                manager.register_object(name, response.interrupt_handlers);
            }
        }
    }
}

pub(crate) fn init_drivers() {
    let drivers = Vec::new();
    DRIVERS.call_once(|| Mutex::new(drivers));
}

pub fn register_driver(driver: Box<dyn PciDriver>) {
    let drivers = DRIVERS.get().expect("PCI drivers not initialized");
    drivers.lock().push(driver);
}
