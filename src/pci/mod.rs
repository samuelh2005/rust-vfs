use crate::{
    acpi::{MCFGEntry, MCFGHeader},
    driver::probe_drivers,
};

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct PCIDeviceHeader {
    pub vendor_id: u16,
    pub device_id: u16,
    pub command: u16,
    pub status: u16,
    pub revision_id: u8,
    pub prog_if: u8,
    pub subclass: u8,
    pub class_code: u8,
    pub cache_line_size: u8,
    pub latency_timer: u8,
    pub header_type: u8,
    pub bist: u8,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct PCIHeaderType0 {
    pub header: PCIDeviceHeader,
    pub bar: [u32; 6],
    pub cardbus_cis_pointer: u32,
    pub subsystem_vendor_id: u16,
    pub subsystem_id: u16,
    pub expansion_rom_base_address: u32,
    pub capabilities_pointer: u8,
    pub reserved: [u8; 7],
    pub interrupt_line: u8,
    pub interrupt_pin: u8,
    pub min_grant: u8,
    pub max_latency: u8,
}

pub fn enumerate_function(device_address: u64, function: u64) {
    let header = unsafe { &*(device_address as *const PCIDeviceHeader) };

    if header.vendor_id == 0xFFFF {
        return;
    }

    let full_header = unsafe { &*(device_address as *const PCIHeaderType0) };

    probe_drivers(&header, &full_header);
}

pub fn enumerate_device(bus_address: u64, device: u64) {
    let device_address = bus_address + (device << 15);

    let header = unsafe { &*(device_address as *const PCIDeviceHeader) };

    if header.vendor_id == 0xFFFF {
        return;
    }

    // Check if multi-function device
    let is_multifunction = (header.header_type & 0x80) != 0;

    let function_count = if is_multifunction { 8 } else { 1 };

    for function in 0..function_count {
        let function_address = device_address + (function << 12);
        enumerate_function(function_address, function);
    }
}

pub fn enumerate_bus(base_address: u64, start_bus: u8, end_bus: u8) {
    for bus in start_bus..=end_bus {
        let bus_address = base_address + ((bus as u64) << 20);

        for device in 0..32 {
            enumerate_device(bus_address, device);
        }
    }
}

pub fn enumerate_pci(mcfg: *const MCFGHeader) {
    let mcfg = unsafe { &*mcfg };

    let entries = (mcfg.header.length as usize - core::mem::size_of::<MCFGHeader>())
        / core::mem::size_of::<MCFGEntry>();

    let entries_ptr = unsafe {
        (mcfg as *const _ as *const u8).add(core::mem::size_of::<MCFGHeader>()) as *const MCFGEntry
    };

    for i in 0..entries {
        let entry = unsafe { &*entries_ptr.add(i) };

        enumerate_bus(
            entry.base_address,
            entry.start_bus_number,
            entry.end_bus_number,
        );
    }
}
