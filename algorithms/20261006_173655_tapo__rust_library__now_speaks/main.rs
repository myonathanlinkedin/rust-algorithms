mod core;
use core::tpap::TpapMessage;

fn main() {
    // Simple runtime sanity checks (panics on failure).
    let request = TpapMessage {
        id: 100,
        method: "turn_on".to_string(),
        payload: "{\"duration\":5}".to_string(),
    };
    let wire = request.encode();
    let parsed = TpapMessage::decode(&wire).expect("should parse own encoding");
    assert_eq!(request, parsed);
}

#[cfg(test)]
mod integration_tests {
    use super::core::tpap::TpapMessage;

    #[test]
    fn integration_encode_decode() {
        let msg = TpapMessage {
            id: 7,
            method: "ping".to_string(),
            payload: "null".to_string(),
        };
        let wire = msg.encode();
        let back = TpapMessage::decode(&wire).unwrap();
        assert_eq!(msg, back);
    }

    #[test]
    fn integration_multiple_messages() {
        let msgs = vec![
            TpapMessage {
                id: 1,
                method: "get_state".to_string(),
                payload: "{}".to_string(),
            },
            TpapMessage {
                id: 2,
                method: "set_state".to_string(),
                payload: "{\"state\":\"off\"}".to_string(),
            },
            TpapMessage {
                id: 3,
                method: "reboot".to_string(),
                payload: "null".to_string(),
            },
        ];

        for original in msgs.iter() {
            let encoded = original.encode();
            let decoded = TpapMessage::decode(&encoded).expect("decode should succeed");
            assert_eq!(original, &decoded);
        }
    }
}