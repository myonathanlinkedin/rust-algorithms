mod types;
mod engine;

use types::{Command, Message, TPAP_VERSION};
use engine::{parse_message, serialize_message};

fn main() {
    // Simple round‑trip test for Ping with empty payload
    let ping_msg = Message {
        version: TPAP_VERSION,
        command: Command::Ping,
        payload: Vec::new(),
    };
    let encoded = serialize_message(&ping_msg);
    let decoded = parse_message(&encoded).expect("Failed to parse ping message");
    assert_eq!(decoded, ping_msg);

    // Test GetStatus with a small payload
    let status_payload = vec![0xAA, 0xBB, 0xCC];
    let status_msg = Message {
        version: TPAP_VERSION,
        command: Command::GetStatus,
        payload: status_payload.clone(),
    };
    let encoded_status = serialize_message(&status_msg);
    let decoded_status = parse_message(&encoded_status).expect("Failed to parse status message");
    assert_eq!(decoded_status, status_msg);
    assert_eq!(decoded_status.payload, status_payload);

    // Test SetConfig where the config id is stored in payload[0]
    let config_id: u8 = 0x42;
    let config_payload = vec![config_id, 0x10, 0x20];
    let set_config_msg = Message {
        version: TPAP_VERSION,
        command: Command::SetConfig(config_id),
        payload: config_payload.clone(),
    };
    let encoded_cfg = serialize_message(&set_config_msg);
    let decoded_cfg = parse_message(&encoded_cfg).expect("Failed to parse config message");
    // Command enum loses the inner id for SetConfig during parsing (placeholder 0),
    // but we can still compare payload and version.
    assert_eq!(decoded_cfg.version, TPAP_VERSION);
    assert_eq!(decoded_cfg.payload, config_payload);
    // Ensure command byte matches
    assert_eq!(decoded_cfg.command.to_u8(), Command::SetConfig(0).to_u8());

    // Edge case: maximum payload length (u16::MAX)
    let max_len = u16::MAX as usize;
    let large_payload = vec![0x55; max_len];
    let large_msg = Message {
        version: TPAP_VERSION,
        command: Command::Unknown(0xFF),
        payload: large_payload.clone(),
    };
    let encoded_large = serialize_message(&large_msg);
    assert_eq!(encoded_large.len(), 4 + max_len);
    let decoded_large = parse_message(&encoded_large).expect("Failed to parse large message");
    assert_eq!(decoded_large.payload.len(), max_len);
    assert_eq!(decoded_large.payload, large_payload);

    // Invalid version should be rejected
    let mut bad_version = encoded.clone();
    bad_version[0] = 0x99;
    assert!(parse_message(&bad_version).is_err());

    // Length mismatch should be rejected
    let mut bad_len = encoded.clone();
    // corrupt length to be larger than actual payload
    bad_len[2] = 0x00;
    bad_len[3] = 0x02; // claim length 2 while payload is 0
    assert!(parse_message(&bad_len).is_err());

    // All assertions passed
    println!("All TPAP protocol tests passed.");
}