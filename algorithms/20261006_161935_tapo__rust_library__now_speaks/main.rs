mod types;
mod engine;

use crate::types::*;
use crate::engine::*;

fn main() {
    // Test 1: Encode and decode a GetStatus request
    let req = Message::Request(Request {
        command: Command::GetStatus,
    });
    let encoded = encode_message(&req);
    let decoded = decode_message(&encoded).expect("decode should succeed");
    assert_eq!(decoded, req);

    // Test 2: Encode and decode a SetPower request (turn on)
    let req_on = Message::Request(Request {
        command: Command::SetPower(true),
    });
    let encoded_on = encode_message(&req_on);
    let decoded_on = decode_message(&encoded_on).expect("decode should succeed");
    assert_eq!(decoded_on, req_on);

    // Test 3: Encode and decode a SetPower request (turn off)
    let req_off = Message::Request(Request {
        command: Command::SetPower(false),
    });
    let encoded_off = encode_message(&req_off);
    let decoded_off = decode_message(&encoded_off).expect("decode should succeed");
    assert_eq!(decoded_off, req_off);

    // Test 4: Encode and decode a successful status response (power on)
    let resp_status_on = Message::Response(Response {
        success: true,
        payload: ResponsePayload::Status(true),
    });
    let encoded_resp_on = encode_message(&resp_status_on);
    let decoded_resp_on = decode_message(&encoded_resp_on).expect("decode should succeed");
    assert_eq!(decoded_resp_on, resp_status_on);

    // Test 5: Encode and decode an acknowledgment response
    let resp_ack = Message::Response(Response {
        success: true,
        payload: ResponsePayload::Ack,
    });
    let encoded_ack = encode_message(&resp_ack);
    let decoded_ack = decode_message(&encoded_ack).expect("decode should succeed");
    assert_eq!(decoded_ack, resp_ack);

    // Edge case: Corrupt start byte
    let mut corrupted = encoded.clone();
    corrupted[0] = 0x00;
    assert!(matches!(
        decode_message(&corrupted),
        Err(DecodeError::InvalidStartByte)
    ));

    // Edge case: Bad checksum
    let mut bad_checksum = encoded.clone();
    let last_idx = bad_checksum.len() - 1;
    bad_checksum[last_idx] ^= 0xFF;
    assert!(matches!(
        decode_message(&bad_checksum),
        Err(DecodeError::ChecksumMismatch)
    ));

    // Edge case: Length mismatch
    let mut bad_len = encoded.clone();
    bad_len[1] = 0x00; // set length to zero
    bad_len[2] = 0x00;
    assert!(matches!(
        decode_message(&bad_len),
        Err(DecodeError::LengthMismatch)
    ));

    // All assertions passed
    println!("All TPAP protocol tests passed.");
}