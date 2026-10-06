mod core;
use core::{Command, TpapMessage, START_BYTE};

fn main() {
    // Simple runtime sanity checks
    let msg = TpapMessage {
        command: Command::GetInfo,
    };
    let encoded = msg.encode();
    assert_eq!(encoded[0], START_BYTE);
    let decoded = TpapMessage::decode(&encoded).expect("decode should succeed");
    assert_eq!(msg, decoded);

    let msg2 = TpapMessage {
        command: Command::SetPower(true),
    };
    let encoded2 = msg2.encode();
    let decoded2 = TpapMessage::decode(&encoded2).expect("decode should succeed");
    assert_eq!(msg2, decoded2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_encode_get_info() {
        let msg = TpapMessage {
            command: Command::GetInfo,
        };
        let encoded = msg.encode();
        // Expected format: [START][LEN][CMD][CHK]
        assert_eq!(encoded.len(), 4);
        assert_eq!(encoded[0], START_BYTE);
        assert_eq!(encoded[1], 3); // cmd + checksum = 2, plus checksum byte = 3
        assert_eq!(encoded[2], Command::GetInfo.id());
        let checksum = encoded[3];
        let calc = TpapMessage::calc_checksum(&encoded[2..3]);
        assert_eq!(checksum, calc);
    }

    #[test]
    fn test_encode_set_power_off() {
        let msg = TpapMessage {
            command: Command::SetPower(false),
        };
        let encoded = msg.encode();
        // [START][LEN][CMD][PAYLOAD][CHK]
        assert_eq!(encoded.len(), 5);
        assert_eq!(encoded[2], Command::SetPower(false).id());
        assert_eq!(encoded[3], 0x00);
        let checksum = encoded[4];
        let calc = TpapMessage::calc_checksum(&encoded[2..4]);
        assert_eq!(checksum, calc);
    }

    #[test]
    fn test_decode_invalid_start() {
        let mut data = vec![0x00, 3, 0x01, 0x00];
        // compute checksum for consistency
        let checksum = TpapMessage::calc_checksum(&data[2..]);
        data.push(checksum);
        let res = TpapMessage::decode(&data);
        assert!(res.is_err());
    }

    #[test]
    fn test_decode_checksum_mismatch() {
        let msg = TpapMessage {
            command: Command::SetPower(true),
        };
        let mut encoded = msg.encode();
        // corrupt checksum
        let last_idx = encoded.len() - 1;
        encoded[last_idx] = encoded[last_idx].wrapping_add(1);
        let res = TpapMessage::decode(&encoded);
        assert!(res.is_err());
    }

    #[test]
    fn round_trip_various_commands() {
        let commands = vec![
            Command::GetInfo,
            Command::SetPower(true),
            Command::SetPower(false),
        ];
        for cmd in commands {
            let msg = TpapMessage { command: cmd.clone() };
            let encoded = msg.encode();
            let decoded = TpapMessage::decode(&encoded).expect("round-trip decode");
            assert_eq!(msg, decoded);
        }
    }
}