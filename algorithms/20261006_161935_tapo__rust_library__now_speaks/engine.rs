pub mod engine {
    use crate::types::*;
    
    const START_BYTE: u8 = 0xAA;

    fn command_id(cmd: &Command) -> u8 {
        match cmd {
            Command::GetStatus => 0x01,
            Command::SetPower(_) => 0x02,
        }
    }

    fn encode_payload(cmd: &Command) -> Vec<u8> {
        match cmd {
            Command::GetStatus => vec![],
            Command::SetPower(on) => vec![if *on { 1 } else { 0 }],
        }
    }

    fn decode_payload(cmd_id: u8, payload: &[u8]) -> Result<Command, DecodeError> {
        match cmd_id {
            0x01 => {
                if !payload.is_empty() {
                    Err(DecodeError::LengthMismatch)
                } else {
                    Ok(Command::GetStatus)
                }
            }
            0x02 => {
                if payload.len() != 1 {
                    Err(DecodeError::LengthMismatch)
                } else {
                    Ok(Command::SetPower(payload[0] != 0))
                }
            }
            other => Err(DecodeError::UnknownCommand(other)),
        }
    }

    fn response_id(payload: &ResponsePayload) -> u8 {
        match payload {
            ResponsePayload::Status(_) => 0x81,
            ResponsePayload::Ack => 0x82,
        }
    }

    fn encode_response_payload(payload: &ResponsePayload) -> Vec<u8> {
        match payload {
            ResponsePayload::Status(on) => vec![if *on { 1 } else { 0 }],
            ResponsePayload::Ack => vec![],
        }
    }

    fn decode_response_payload(resp_id: u8, payload: &[u8]) -> Result<ResponsePayload, DecodeError> {
        match resp_id {
            0x81 => {
                if payload.len() != 1 {
                    Err(DecodeError::LengthMismatch)
                } else {
                    Ok(ResponsePayload::Status(payload[0] != 0))
                }
            }
            0x82 => {
                if !payload.is_empty() {
                    Err(DecodeError::LengthMismatch)
                } else {
                    Ok(ResponsePayload::Ack)
                }
            }
            other => Err(DecodeError::UnknownCommand(other)),
        }
    }

    fn compute_checksum(data: &[u8]) -> u8 {
        data.iter().fold(0u8, |acc, &b| acc ^ b)
    }

    pub fn encode_message(msg: &Message) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(START_BYTE);
        // placeholder for length (u16)
        buf.extend_from_slice(&[0, 0]);
        match msg {
            Message::Request(req) => {
                let cmd_id = command_id(&req.command);
                let payload = encode_payload(&req.command);
                buf.push(cmd_id);
                buf.extend_from_slice(&payload);
            }
            Message::Response(resp) => {
                let resp_id = response_id(&resp.payload);
                let payload = encode_response_payload(&resp.payload);
                buf.push(resp_id);
                buf.extend_from_slice(&payload);
                // success flag as a single byte after payload
                buf.push(if resp.success { 1 } else { 0 });
            }
        }
        // fill length (excluding start byte and length field itself)
        let length = (buf.len() - 3) as u16;
        let len_bytes = length.to_be_bytes();
        buf[1] = len_bytes[0];
        buf[2] = len_bytes[1];
        // checksum over length, command/id, payload, (and success flag for response)
        let checksum = compute_checksum(&buf[1..]);
        buf.push(checksum);
        buf
    }

    pub fn decode_message(data: &[u8]) -> Result<Message, DecodeError> {
        if data.is_empty() {
            return Err(DecodeError::UnexpectedEof);
        }
        if data[0] != START_BYTE {
            return Err(DecodeError::InvalidStartByte);
        }
        if data.len() < 4 {
            return Err(DecodeError::UnexpectedEof);
        }
        let length = u16::from_be_bytes([data[1], data[2]]) as usize;
        let expected_total = 3 + length + 1; // start+len+payload+checksum
        if data.len() != expected_total {
            return Err(DecodeError::LengthMismatch);
        }
        let checksum = data[data.len() - 1];
        let computed = compute_checksum(&data[1..data.len() - 1]);
        if checksum != computed {
            return Err(DecodeError::ChecksumMismatch);
        }
        // payload starts at index 3
        let payload = &data[3..data.len() - 1];
        if payload.is_empty() {
            return Err(DecodeError::UnexpectedEof);
        }
        let id = payload[0];
        if id & 0x80 == 0 {
            // request
            let cmd = decode_payload(id, &payload[1..])?;
            Ok(Message::Request(Request { command: cmd }))
        } else {
            // response
            if payload.len() < 2 {
                return Err(DecodeError::UnexpectedEof);
            }
            let success = payload[payload.len() - 2] != 0;
            let resp_payload = decode_response_payload(id, &payload[1..payload.len() - 2])?;
            Ok(Message::Response(Response {
                success,
                payload: resp_payload,
            }))
        }
    }
}

// Re-export for easier access
pub use engine::*;