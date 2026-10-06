pub const TPAP_VERSION: u8 = 1;

#[derive(Debug, Clone, PartialEq)]
pub struct TpapMessage {
    pub version: u8,
    pub command: u8,
    pub payload: Vec<u8>,

}

impl TpapMessage {
    /// Encode the message into the TPAP wire format:
    /// [version][command][len_hi][len_lo][payload...][checksum]
    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(4 + self.payload.len() + 1);
        buf.push(self.version);
        buf.push(self.command);
        let len = self.payload.len() as u16;
        buf.push((len >> 8) as u8);
        buf.push((len & 0xFF) as u8);
        buf.extend_from_slice(&self.payload);
        let chk = checksum(&buf);
        buf.push(chk);
        buf
    }

    /// Decode a byte slice into a `TpapMessage`. Returns an error string on failure.
    pub fn decode(data: &[u8]) -> Result<Self, String> {
        if data.len() < 5 {
            return Err("data too short".into());
        }
        let version = data[0];
        let command = data[1];
        let len = ((data[2] as u16) << 8) | data[3] as u16;
        let expected_len = 4usize + len as usize + 1; // header + payload + checksum
        if data.len() != expected_len {
            return Err(format!(
                "length mismatch: expected {}, got {}",
                expected_len,
                data.len()
            ));
        }
        let payload_start = 4usize;
        let payload_end = payload_start + len as usize;
        let payload = data[payload_start..payload_end].to_vec();
        let received_chk = data[payload_end];
        let computed_chk = checksum(&data[0..payload_end]);
        if received_chk != computed_chk {
            return Err(format!(
                "checksum mismatch: expected {:02X}, got {:02X}",
                computed_chk, received_chk
            ));
        }
        Ok(TpapMessage {
            version,
            command,
            payload,
        })
    }
}

/// Simple additive checksum (mod 256) over the provided slice.
fn checksum(data: &[u8]) -> u8 {
    data.iter().fold(0u8, |acc, &b| acc.wrapping_add(b))
}

// Helper constructors for common commands (example purposes)
impl TpapMessage {
    pub fn get_device_info() -> Self {
        TpapMessage {
            version: TPAP_VERSION,
            command: 0x01,
            payload: Vec::new(),
        }
    }

    pub fn set_power(state: bool) -> Self {
        TpapMessage {
            version: TPAP_VERSION,
            command: 0x02,
            payload: vec![if state { 1 } else { 0 }],
        }
    }
}