use crate::object::Object;

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

pub type ObjectResult<T> = Result<T, OperationError>;
pub type OperationHandler = fn(&Object, ObjectOperation, ObjectData) -> ObjectResult<ObjectData>;
