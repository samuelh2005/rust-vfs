#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationError {
    /// The specified object was not found.
    NotFound = 1,
    /// The caller does not have permission to perform the operation on the
    /// specified object.
    PermissionDenied = 2,
    /// The specified operation is not supported by the object.
    UnsupportedOperation = 3,
}

impl core::convert::TryFrom<usize> for OperationError {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(OperationError::NotFound),
            2 => Ok(OperationError::PermissionDenied),
            3 => Ok(OperationError::UnsupportedOperation),
            _ => Err(()),
        }
    }
}

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectOperation {
    Read = 0,
    Write = 1,
}

impl core::convert::TryFrom<usize> for ObjectOperation {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ObjectOperation::Read),
            1 => Ok(ObjectOperation::Write),
            _ => Err(()),
        }
    }
}

pub enum ObjectData {
    Bytes(*const u8, usize),
    None,
}

pub type ObjectHandle = u64;
pub type ObjectResult<T> = Result<T, OperationError>;
pub type OperationHandler = fn(&Object, ObjectOperation, ObjectData) -> ObjectResult<ObjectData>;
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

pub struct Object {
    handler: OperationHandler,
    name: &'static str,
    obj_type: ObjectType,
}

impl Object {
    pub fn new(name: &'static str, obj_type: ObjectType, handler: OperationHandler) -> Self {
        Object { handler, name, obj_type }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn obj_type(&self) -> ObjectType {
        self.obj_type
    }

    pub fn handle_operation(
        &self,
        operation: ObjectOperation,
        data: ObjectData,
    ) -> ObjectResult<ObjectData> {
        (self.handler)(self, operation, data)
    }
}
