use crate::object::command::OperationError;

pub enum DisplayCommands {
    ConsoleRead = 0,
    ConsoleWrite = 1,
}

impl TryFrom<usize> for DisplayCommands {
    type Error = OperationError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DisplayCommands::ConsoleRead),
            1 => Ok(DisplayCommands::ConsoleWrite),
            _ => Err(OperationError::UnsupportedOperation),
        }
    }
}
