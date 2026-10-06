use crate::types::{Command, Message, TPAP_VERSION};

pub fn serialize_message(msg: &Message) -> Vec<u8> {
    // Header: version (1 byte), command (1 byte), length (2 bytes, big endian)
    let mut buf = Vec::with_capacity(4 + msg.payload.len());
    buf.push(msg.version);
    buf.push(msg.command.to_u8());
    let length = msg.payload.len() as u16;
    buf.push((length >> 8) as u8);
    buf.push((length & 0xFF) as u8);
    buf.extend_from_slice(&msg.payload);
    buf
}

pub fn parse_message(data: &[u8]) -> Result<Message, String> {
    if data.len() < 4 {
        return Err("Data too short for TPAP header".into());
    }
    let version = data[0];
    if version != TPAP_VERSION {
        return Err(format!("Unsupported TPAP version: {}", version));
    }
    let command_byte = data[1];
    let command = Command::from_u8(command_byte);
    let length = ((data[2] as u16) << 8) | (data[3] as u16);
    let expected_len = 4usize + length as usize;
    if data.len() != expected_len {
        return Err(format!(
            "Length mismatch: header says {}, but actual payload size is {}",
            length,
            data.len() - 4
        ));
    }
    let payload = data[4..].to_vec();
    Ok(Message {
        version,
        command,
        payload,
    })
}