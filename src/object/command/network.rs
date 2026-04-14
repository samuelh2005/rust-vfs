use crate::object::command::OperationError;

pub enum NetworkCommands {
    SendPacket = 0,
    ReceivePacket = 1,
}

impl TryFrom<usize> for NetworkCommands {
    type Error = OperationError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(NetworkCommands::SendPacket),
            1 => Ok(NetworkCommands::ReceivePacket),
            _ => Err(OperationError::UnsupportedOperation),
        }
    }
}
