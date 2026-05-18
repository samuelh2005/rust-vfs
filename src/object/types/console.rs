use crate::object::command::CommandError;

pub enum ConsoleCommands {
    ConsoleRead = 0,
    ConsoleWrite = 1,
}

impl TryFrom<usize> for ConsoleCommands {
    type Error = CommandError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ConsoleCommands::ConsoleRead),
            1 => Ok(ConsoleCommands::ConsoleWrite),
            _ => Err(CommandError::UnsupportedOperation),
        }
    }
}

#[repr(usize)]
pub enum ConsoleEvents {
    ConsoleInputAvailable = 0,
    ConsoleReadyForOutput = 1,
}
