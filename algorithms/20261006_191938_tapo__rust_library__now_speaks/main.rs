mod core;
use core::{TpapMessage, TpapError};

fn run_basic_asserts() {
    // Empty payload
    let msg = TpapMessage {
        command: 0x01,
        payload: Vec::new(),
    };
    let bytes = msg.to_bytes();
    assert_eq!(bytes, vec![0x00, 0x01, 0x01]);
    let parsed = TpapMessage::from_bytes(&bytes).unwrap();
    assert_eq!(msg, parsed);

    // Non‑empty payload
    let payload = b"hello".to_vec();
    let msg2 = TpapMessage {
        command: 0xA5,
        payload: payload.clone(),
    };
    let bytes2 = msg2.to_bytes();
    // length = 1 (cmd) + 5 (payload) = 6 => 0x0006
    assert_eq!(bytes2, vec![0x00, 0x06, 0xA5, b'h', b'e', b'l', b'l', b'o']);
    let parsed2 = TpapMessage::from_bytes(&bytes2).unwrap();
    assert_eq!(msg2, parsed2);
    assert_eq!(parsed2.payload, payload);

    // Large payload (max u16 size)
    let large_payload = vec![0xFF; 65534]; // 65534 + 1 command = 65535 = u16::MAX
    let msg3 = TpapMessage {
        command: 0xFF,
        payload: large_payload.clone(),
    };
    let bytes3 = msg3.to_bytes();
    assert_eq!(bytes3.len(), 2 + 65535);
    // Verify length bytes are 0xFF, 0xFF
    assert_eq!(&bytes3[0..2], &[0xFF, 0xFF]);
    let parsed3 = TpapMessage::from_bytes(&bytes3).unwrap();
    assert_eq!(parsed3.command, 0xFF);
    assert_eq!(parsed3.payload, large_payload);
}

fn run_error_cases() {
    // Too short data
    let err = TpapMessage::from_bytes(&[0x00]).err().unwrap();
    assert_eq!(err, TpapError::InsufficientData);

    // Length field zero (invalid)
    let err2 = TpapMessage::from_bytes(&[0x00, 0x00, 0x01]).err().unwrap();
    assert_eq!(err2, TpapError::InvalidLength);

    // Declared length longer than actual data
    let err3 = TpapMessage::from_bytes(&[0x00, 0x05, 0x02, 0xAA, 0xBB]).err().unwrap();
    assert_eq!(err3, TpapError::LengthMismatch);
}

fn main() {
    run_basic_asserts();
    run_error_cases();
    println!("All TPAP protocol unit tests passed.");
}

// Unit tests using the built‑in test harness
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip_empty() {
        let msg = TpapMessage {
            command: 0x00,
            payload: Vec::new(),
        };
        let bytes = msg.to_bytes();
        let decoded = TpapMessage::from_bytes(&bytes).unwrap();
        assert_eq!(msg, decoded);
    }

    #[test]
    fn test_roundtrip_random_payload() {
        let payload: Vec<u8> = (0..128).map(|i| (i * 3 % 256) as u8).collect();
        let msg = TpapMessage {
            command: 0x7E,
            payload: payload.clone(),
        };
        let bytes = msg.to_bytes();
        let decoded = TpapMessage::from_bytes(&bytes).unwrap();
        assert_eq!(decoded.command, 0x7E);
        assert_eq!(decoded.payload, payload);
    }

    #[test]
    fn test_invalid_length_field() {
        // Length says 2, but only 1 byte after header
        let data = vec![0x00, 0x02, 0x10];
        let err = TpapMessage::from_bytes(&data).err().unwrap();
        assert_eq!(err, TpapError::LengthMismatch);
    }

    #[test]
    fn test_maximum_message() {
        let payload = vec![0xAB; 65534];
        let msg = TpapMessage {
            command: 0x01,
            payload,
        };
        let bytes = msg.to_bytes();
        assert_eq!(bytes[0], 0xFF);
        assert_eq!(bytes[1], 0xFF);
        let decoded = TpapMessage::from_bytes(&bytes).unwrap();
        assert_eq!(decoded.command, 0x01);
        assert_eq!(decoded.payload.len(), 65534);
    }
}