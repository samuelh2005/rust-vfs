#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PCIType {
    Unclassified,
    Storage,
    Network,
    Display,
    Multimedia,
    Memory,
    Bridge,
    Comm,
    System,
    Input,
    Docking,
    Processor,
    SerialBus,
    Wireless,
    Io,
    Satcom,
    Crypto,
    Data,
    Vendor,
    Unknown,
}

impl PCIType {
    pub fn label(&self) -> &'static str {
        match self {
            PCIType::Unclassified => "unclassified",
            PCIType::Storage => "storage",
            PCIType::Network => "network",
            PCIType::Display => "display",
            PCIType::Multimedia => "multimedia",
            PCIType::Memory => "memory",
            PCIType::Bridge => "bridge",
            PCIType::Comm => "comm",
            PCIType::System => "system",
            PCIType::Input => "input",
            PCIType::Docking => "docking",
            PCIType::Processor => "processor",
            PCIType::SerialBus => "serialbus",
            PCIType::Wireless => "wireless",
            PCIType::Io => "io",
            PCIType::Satcom => "satcom",
            PCIType::Crypto => "crypto",
            PCIType::Data => "data",
            PCIType::Vendor => "vendor",
            PCIType::Unknown => "unknown",
        }
    }
}

pub fn class_type_from_code(class: u8) -> PCIType {
    match class {
        0x00 => PCIType::Unclassified,
        0x01 => PCIType::Storage,
        0x02 => PCIType::Network,
        0x03 => PCIType::Display,
        0x04 => PCIType::Multimedia,
        0x05 => PCIType::Memory,
        0x06 => PCIType::Bridge,
        0x07 => PCIType::Comm,
        0x08 => PCIType::System,
        0x09 => PCIType::Input,
        0x0A => PCIType::Docking,
        0x0B => PCIType::Processor,
        0x0C => PCIType::SerialBus,
        0x0D => PCIType::Wireless,
        0x0E => PCIType::Io,
        0x0F => PCIType::Satcom,
        0x10 => PCIType::Crypto,
        0x11 => PCIType::Data,
        0xFF => PCIType::Vendor,
        _ => PCIType::Unknown,
    }
}
