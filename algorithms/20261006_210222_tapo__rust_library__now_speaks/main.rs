mod core;
use core::TpapMessage;

/// Simulate a TPAP exchange. In a real client this would involve network I/O.
fn simulate_exchange(request: &TpapMessage) -> TpapMessage {
    TpapMessage {
        command_id: request.command_id.wrapping_add(1),
        payload: format!("response to {}", request.payload),
    }
}

fn main() {
    // Basic round‑trip test using `assert!`
    let request = TpapMessage {
        command_id: 0x01,
        payload: "{\"method\":\"get_device_info\"}".to_string(),
    };
    let encoded = request.encode();
    let decoded = TpapMessage::decode(&encoded).expect("decode must succeed");
    assert_eq!(request, decoded);

    let response = simulate_exchange(&request);
    assert_eq!(response.command_id, request.command_id + 1);
    assert!(response.payload.contains("response to"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_roundtrip() {
        let msg = TpapMessage {
            command_id: 0x10,
            payload: "test payload".to_string(),
        };
        let bytes = msg.encode();
        let decoded = TpapMessage::decode(&bytes).unwrap();
        assert_eq!(msg, decoded);
    }

    #[test]
    fn decode_invalid_length() {
        // Declared length 5, but only 2 bytes follow.
        let data = vec![0, 5, 0, 1, b'h', b'i'];
        let err = TpapMessage::decode(&data).unwrap_err();
        assert_eq!(err, "payload length mismatch");
    }

    #[test]
    fn decode_non_utf8_payload() {
        // Payload contains invalid UTF‑8 (0xFF).
        let mut data = vec![0, 1, 0, 1, 0xFF];
        let err = TpapMessage::decode(&data).unwrap_err();
        assert_eq!(err, "invalid UTF-8 payload");
    }

    #[test]
    fn simulate_exchange_behaviour() {
        let req = TpapMessage {
            command_id: 42,
            payload: "ping".to_string(),
        };
        let resp = simulate_exchange(&req);
        assert_eq!(resp.command_id, 43);
        assert_eq!(resp.payload, "response to ping");
    }
}