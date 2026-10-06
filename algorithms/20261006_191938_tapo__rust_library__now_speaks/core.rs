use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq)]
pub struct TpapMessage {
    pub command: u8,
    pub payload: Vec<u8>,

}

#[derive(Debug, PartialEq)]
pub enum TpapError {
    InsufficientData,
    LengthMismatch,
    InvalidLength,
}

impl TpapMessage {
    /// Serialize the message into a byte vector.
    /// Format: [len_hi, len_lo, command, payload...]
    /// `len` is a big‑endian u16 representing command + payload length.
    pub fn to_bytes(&self) -> Vec<u8> {
        let payload_len = self.payload.len();
        let total_len = 1usize + payload_len; // command + payload
        assert!(total_len <= u16::MAX as usize, "Message too large");
        let len_u16 = u16::try_from(total_len).unwrap();

        let mut buf = Vec::with_capacity(2 + total_len);
        buf.push((len_u16 >> 8) as u8);
        buf.push((len_u16 & 0xFF) as u8);
        buf.push(self.command);
        buf.extend_from_slice(&self.payload);
        buf
    }

    /// Deserialize a message from a byte slice.
    pub fn from_bytes(data: &[u8]) -> Result<Self, TpapError> {
        if data.len() < 3 {
            return Err(TpapError::InsufficientData);
        }
        let len = ((data[0] as u16) << 8) | (data[1] as u16);
        let len_usize = len as usize;
        if len_usize == 0 {
            return Err(TpapError::InvalidLength);
        }
        if data.len() < 2 + len_usize {
            return Err(TpapError::LengthMismatch);
        }
        let command = data[2];
        let payload = data[3..2 + len_usize].to_vec();
        Ok(TpapMessage { command, payload })
    }
}