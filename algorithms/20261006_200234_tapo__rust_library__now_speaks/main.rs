mod core;
use core::{TpapMessage, TPAP_VERSION};

fn main() {
    // Basic sanity checks executed at runtime.
    let msg = TpapMessage::set_power(true);
    let encoded = msg.encode();
    let decoded = TpapMessage::decode(&encoded).expect("decode should succeed");
    assert_eq!(msg, decoded);
    // Intentional checksum failure test.
    let mut corrupted = encoded.clone();
    let last_idx = corrupted.len() - 1;
    corrupted[last_idx] ^= 0xFF; // flip bits
    assert!(TpapMessage::decode(&corrupted).is_err());
    println!("All runtime assertions passed.");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn roundtrip_basic() {
        let msg = TpapMessage {
            version: TPAP_VERSION,
            command: 0x10,
            payload: vec![0xDE, 0xAD, 0xBE, 0xEF],
        };
        let bytes = msg.encode();
        let parsed = TpapMessage::decode(&bytes).expect("should decode");
        assert_eq!(msg, parsed);
    }

    #[test]
    fn empty_payload() {
        let msg = TpapMessage::get_device_info();
        let bytes = msg.encode();
        let parsed = TpapMessage::decode(&bytes).unwrap();
        assert_eq!(msg, parsed);
        assert!(parsed.payload.is_empty());
    }

    #[test]
    fn checksum_error() {
        let mut bytes = TpapMessage::set_power(false).encode();
        // corrupt a payload byte
        if bytes.len() > 5 {
            let idx = 4usize; // first payload byte
            bytes[idx] ^= 0xAA;
        }
        let result = TpapMessage::decode(&bytes);
        assert!(result.is_err());
        if let Err(msg) = result {
            assert!(msg.contains("checksum"));
        }
    }

    #[test]
    fn length_mismatch() {
        let mut bytes = TpapMessage::set_power(true).encode();
        // truncate the message
        bytes.pop();
        let result = TpapMessage::decode(&bytes);
        assert!(result.is_err());
        if let Err(msg) = result {
            assert!(msg.contains("length"));
        }
    }
}