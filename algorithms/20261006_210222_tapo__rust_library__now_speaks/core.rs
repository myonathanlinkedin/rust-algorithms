use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq)]
pub struct TpapMessage {
    pub command_id: u16,
    pub payload: String,

}

impl TpapMessage {
    /// Encode the message as:
    /// [payload_len: u16][command_id: u16][payload bytes]
    pub fn encode(&self) -> Vec<u8> {
        let payload_bytes = self.payload.as_bytes();
        let length = payload_bytes.len() as u16;
        let mut buf = Vec::with_capacity(4 + payload_bytes.len());
        buf.extend_from_slice(&length.to_be_bytes());
        buf.extend_from_slice(&self.command_id.to_be_bytes());
        buf.extend_from_slice(payload_bytes);
        buf
    }

    /// Decode a byte slice into a `TpapMessage`.
    /// Returns an error string on failure.
    pub fn decode(data: &[u8]) -> Result<Self, String> {
        if data.len() < 4 {
            return Err("data too short".into());
        }
        let length = u16::from_be_bytes(data[0..2].try_into().unwrap()) as usize;
        let command_id = u16::from_be_bytes(data[2..4].try_into().unwrap());
        if data.len() < 4 + length {
            return Err("payload length mismatch".into());
        }
        let payload_slice = &data[4..4 + length];
        let payload = std::str::from_utf8(payload_slice)
            .map_err(|_| "invalid UTF-8 payload".to_string())?
            .to_string();
        Ok(TpapMessage { command_id, payload })
    }
}