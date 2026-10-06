pub const START_BYTE: u8 = 0xAA;

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    GetInfo,
    SetPower(bool),
}

impl Command {
    fn id(&self) -> u8 {
        match self {
            Command::GetInfo => 0x01,
            Command::SetPower(_) => 0x02,
        }
    }

    fn from_id(id: u8, payload: &[u8]) -> Result<Self, String> {
        match id {
            0x01 => {
                if !payload.is_empty() {
                    return Err("GetInfo should have empty payload".into());
                }
                Ok(Command::GetInfo)
            }
            0x02 => {
                if payload.len() != 1 {
                    return Err("SetPower payload length must be 1".into());
                }
                match payload[0] {
                    0x00 => Ok(Command::SetPower(false)),
                    0x01 => Ok(Command::SetPower(true)),
                    _ => Err("Invalid power state".into()),
                }
            }
            _ => Err(format!("Unknown command id: {:#04x}", id)),
        }
    }

    fn payload(&self) -> Vec<u8> {
        match self {
            Command::GetInfo => Vec::new(),
            Command::SetPower(state) => vec![if *state { 0x01 } else { 0x00 }],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TpapMessage {
    pub command: Command,

}

impl TpapMessage {
    pub fn encode(&self) -> Vec<u8> {
        let payload = self.command.payload();
        let cmd_id = self.command.id();
        // Length = cmd + payload + checksum
        let len: u8 = (1 + payload.len() + 1) as u8;
        let mut buf = Vec::with_capacity(2 + len as usize);
        buf.push(START_BYTE);
        buf.push(len);
        buf.push(cmd_id);
        buf.extend_from_slice(&payload);
        let checksum = Self::calc_checksum(&buf[2..]); // over cmd+payload
        buf.push(checksum);
        buf
    }

    pub fn decode(data: &[u8]) -> Result<Self, String> {
        if data.len() < 4 {
            return Err("Data too short".into());
        }
        if data[0] != START_BYTE {
            return Err("Invalid start byte".into());
        }
        let len = data[1] as usize;
        if data.len() != 2 + len {
            return Err("Length mismatch".into());
        }
        let checksum = data[2 + len - 1];
        let checksum_calc = Self::calc_checksum(&data[2..2 + len - 1]);
        if checksum != checksum_calc {
            return Err("Checksum mismatch".into());
        }
        let cmd_id = data[2];
        let payload = &data[3..2 + len - 1];
        let command = Command::from_id(cmd_id, payload)?;
        Ok(TpapMessage { command })
    }

    fn calc_checksum(slice: &[u8]) -> u8 {
        slice.iter().fold(0u8, |acc, &b| acc.wrapping_add(b))
    }
}