pub const TPAP_VERSION: u8 = 1;

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Ping,
    GetStatus,
    SetConfig(u8), // config id
    Unknown(u8),
}

impl Command {
    pub fn from_u8(byte: u8) -> Self {
        match byte {
            0x01 => Command::Ping,
            0x02 => Command::GetStatus,
            0x03 => Command::SetConfig(0), // placeholder, actual id stored in payload
            other => Command::Unknown(other),
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            Command::Ping => 0x01,
            Command::GetStatus => 0x02,
            Command::SetConfig(_) => 0x03,
            Command::Unknown(v) => *v,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub version: u8,
    pub command: Command,
    pub payload: Vec<u8>,

}