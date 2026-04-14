#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ObjectType {
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
    Console,
    Unknown,
}

impl ObjectType {
    pub fn label(&self) -> &'static str {
        match self {
            ObjectType::Unclassified => "unclassified",
            ObjectType::Storage => "storage",
            ObjectType::Network => "network",
            ObjectType::Display => "display",
            ObjectType::Multimedia => "multimedia",
            ObjectType::Memory => "memory",
            ObjectType::Bridge => "bridge",
            ObjectType::Comm => "comm",
            ObjectType::System => "system",
            ObjectType::Input => "input",
            ObjectType::Docking => "docking",
            ObjectType::Processor => "processor",
            ObjectType::SerialBus => "serialbus",
            ObjectType::Wireless => "wireless",
            ObjectType::Io => "io",
            ObjectType::Satcom => "satcom",
            ObjectType::Crypto => "crypto",
            ObjectType::Data => "data",
            ObjectType::Vendor => "vendor",
            ObjectType::Console => "console",
            ObjectType::Unknown => "unknown",
        }
    }
}

pub fn class_type_from_code(class: u8) -> ObjectType {
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
