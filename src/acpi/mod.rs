#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct RSDP2 {
    pub signature: [u8; 8],
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub revision: u8,
    pub rsdt_address: u32,
    pub length: u32,
    pub xsdt_address: u64,
    pub extended_checksum: u8,
    pub reserved: [u8; 3],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SDTHeader {
    pub signature: [u8; 4],
    pub length: u32,
    pub revision: u8,
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub oem_table_id: [u8; 8],
    pub oem_revision: u32,
    pub creator_id: u32,
    pub creator_revision: u32,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct MCFGHeader {
    pub header: SDTHeader,
    pub reserved: u64,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct MCFGEntry {
    pub base_address: u64,
    pub segment_group_number: u16,
    pub start_bus_number: u8,
    pub end_bus_number: u8,
    pub reserved: [u8; 4],
}

pub unsafe fn find_table_by_sig(
    sdt_header: &SDTHeader,
    signature: [u8; 4],
) -> Option<&'static SDTHeader> {
    let base = sdt_header as *const _ as *const u8;

    let entries = (sdt_header.length as usize - core::mem::size_of::<SDTHeader>()) / 8;
    let entries_ptr = unsafe { base.add(core::mem::size_of::<SDTHeader>()) as *const u64 };

    for i in 0..entries {
        let entry_ptr = unsafe { entries_ptr.add(i) };
        let hdr_ptr = unsafe { *entry_ptr as *const SDTHeader };

        let sig = unsafe {
            core::ptr::read_unaligned(
                core::ptr::addr_of!((*hdr_ptr).signature)
            )
        };

        if sig == signature {
            return Some(unsafe { &*hdr_ptr });
        }
    }

    None
}

pub unsafe fn enumerate_acpi(xsdt: &SDTHeader) {
    if let Some(mcfg_hdr) = unsafe { find_table_by_sig(xsdt, *b"MCFG") } {
        let mcfg = mcfg_hdr as *const _ as *const MCFGHeader;
        crate::pci::enumerate_pci(mcfg);
    }
}