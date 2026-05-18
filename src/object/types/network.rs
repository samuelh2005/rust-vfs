use crate::object::command::CommandError;

pub enum NetworkCommands {
    SendPacket = 0,
    ReceivePacket = 1,
}

impl TryFrom<usize> for NetworkCommands {
    type Error = CommandError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(NetworkCommands::SendPacket),
            1 => Ok(NetworkCommands::ReceivePacket),
            _ => Err(CommandError::UnsupportedOperation),
        }
    }
}

pub enum NetworkEvents {
    PacketReceived = 0,
    PacketSent = 1,
}

impl TryFrom<usize> for NetworkEvents {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(NetworkEvents::PacketReceived),
            1 => Ok(NetworkEvents::PacketSent),
            _ => Err(()),
        }
    }
}
