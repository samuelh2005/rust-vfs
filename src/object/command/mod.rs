use crate::object::ObjectID;

pub mod console;
pub mod network;

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandError {
    /// The specified object was not found.
    NotFound = 1,
    /// The caller does not have permission to perform the operation on the
    /// specified object.
    PermissionDenied = 2,
    /// The specified operation is not supported by the object.
    UnsupportedOperation = 3,
}

impl core::convert::TryFrom<usize> for CommandError {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(CommandError::NotFound),
            2 => Ok(CommandError::PermissionDenied),
            3 => Ok(CommandError::UnsupportedOperation),
            _ => Err(()),
        }
    }
}

pub enum CommandData {
    Bytes(*const u8, usize),
    None,
}

pub type CommandResult<T> = Result<T, CommandError>;
pub type ObjectCommandHandler = fn(&ObjectID, CommandID, CommandData) -> CommandResult<CommandData>;
pub type CommandID = usize;
