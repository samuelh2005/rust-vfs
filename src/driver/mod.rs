use spin::{Mutex, Once};
use alloc::{boxed::Box, vec::Vec};

use crate::pci::{PCIDeviceHeader, PCIHeaderType0};
use crate::object::{OperationHandler, ObjectType};
use crate::OBJECT_MANAGER;
fn class_type_from_code(class: u8) -> ObjectType {
    match class {
        0x00 => ObjectType::Unclassified,
        0x01 => ObjectType::Storage,
        0x02 => ObjectType::Network,
        0x03 => ObjectType::Display,
        0x04 => ObjectType::Multimedia,
        0x05 => ObjectType::Memory,
        0x06 => ObjectType::Bridge,
        0x07 => ObjectType::Comm,
        0x08 => ObjectType::System,
        0x09 => ObjectType::Input,
        0x0A => ObjectType::Docking,
        0x0B => ObjectType::Processor,
        0x0C => ObjectType::SerialBus,
        0x0D => ObjectType::Wireless,
        0x0E => ObjectType::Io,
        0x0F => ObjectType::Satcom,
        0x10 => ObjectType::Crypto,
        0x11 => ObjectType::Data,
        0xFF => ObjectType::Vendor,
        _ => ObjectType::Unknown,
    }
}

pub trait PciDriver: Send {
    fn supports(&self, vendor_id: u16, device_id: u16, class: u8, subclass: u8) -> bool;

    /// Called when a matching device is found. Return an `OperationHandler`
    /// to expose the device via the VFS object manager, or `None` if the
    /// driver wants to handle the device without exposing an object.
    fn init(&self, pci: &PCIDeviceHeader, func: &PCIHeaderType0) -> Option<OperationHandler>;
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

    for driver in drivers.iter() {
        if driver.supports(vendor, device, class, subclass) {
            if let Some(handler) = driver.init(pci_header, pci_function) {

                // Create the canonical `<type><count>` name. Determine the
                // next unused numeric index for this class label by
                // inspecting registered object names in the manager.
                let class_type = class_type_from_code(class);

                let manager_mutex = OBJECT_MANAGER.get().expect("Object manager not initialized");
                let mut manager = manager_mutex.lock();

                // Use the manager's efficient allocator for type-based names.
                let _name_static = manager.register_object(handler, class_type);
            }
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