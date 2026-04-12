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
    Write = 1
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

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectType {
    Console = 0,
}

impl core::convert::TryFrom<usize> for ObjectType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ObjectType::Console),
            _ => Err(()),
        }
    }
}

pub type ObjectId = u64;
pub type ObjectResult<T> = Result<T, OperationError>;
pub type OperationHandler = fn(ObjectId, ObjectOperation, Option<*const [u8]>) -> ObjectResult<Option<*const [u8]>>;

pub struct Object {
    id: ObjectId,
    handler: OperationHandler,
    object_type: ObjectType,
}

impl Object {
    pub fn new(id: ObjectId, object_type: ObjectType, handler: OperationHandler) -> Self {
        Object { id, handler, object_type }
    }

    pub fn id(&self) -> ObjectId {
        self.id
    }

    pub fn object_type(&self) -> ObjectType {
        self.object_type
    }

    pub fn handle_operation(&self, operation: ObjectOperation, data: Option<*const [u8]>) -> ObjectResult<Option<*const [u8]>> {
        (self.handler)(self.id, operation, data)
    }
}
