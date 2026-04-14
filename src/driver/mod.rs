use alloc::{boxed::Box, vec::Vec};
use spin::{Mutex, Once};

use crate::OBJECT_MANAGER;
use crate::object::command::ObjectCommandHandler;
use crate::object::types::class_type_from_code;
use crate::pci::{PCIDeviceHeader, PCIHeaderType0};

pub trait PciDriver: Send {
    fn supports(&self, vendor_id: u16, device_id: u16, class: u8, subclass: u8) -> bool;

    /// Called when a matching device is found. Return an `OperationHandler`
    /// to expose the device via the VFS object manager, or `None` if the
    /// driver wants to handle the device without exposing an object.
    fn init(
        &self,
        name: &'static str,
        pci: &PCIDeviceHeader,
        func: &PCIHeaderType0,
    ) -> Option<ObjectCommandHandler>;
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
            // Create the canonical `<type><count>` name. Determine the
            // next unused numeric index for this class label by
            // inspecting registered object names in the manager.
            let class_type = class_type_from_code(class);

            let name_static = manager.register_object(class_type);
            if let Some(handler) = driver.init(name_static, pci_header, pci_function) {
                manager.set_object_handler(name_static, handler);
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
