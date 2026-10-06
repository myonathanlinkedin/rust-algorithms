pub mod tpap {
    use std::convert::TryFrom;
    use std::error::Error;
    use std::fmt;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Command {
        GetInfo,
        SetPower(bool),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Request {
        pub seq: u32,
        pub command: Command,

    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum ResponsePayload {
        Info { model: String, firmware: String },
        PowerState(bool),
        Ack,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Response {
        pub seq: u32,
        pub payload: ResponsePayload,

    }

    #[derive(Debug)]
    pub enum DecodeError {
        UnexpectedEof,
        InvalidMessageType(u8),
        UnknownCommandId(u8),
        UnknownPayloadId(u8),
        Utf8Error(std::string::FromUtf8Error),
    }

    impl fmt::Display for DecodeError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                DecodeError::UnexpectedEof => write!(f, "unexpected end of input"),
                DecodeError::InvalidMessageType(t) => write!(f, "invalid message type: {:#x}", t),
                DecodeError::UnknownCommandId(id) => write!(f, "unknown command id: {:#x}", id),
                DecodeError::UnknownPayloadId(id) => write!(f, "unknown payload id: {:#x}", id),
                DecodeError::Utf8Error(e) => write!(f, "utf8 error: {}", e),
            }
        }
    }

    impl Error for DecodeError {}

    const MSG_TYPE_REQUEST: u8 = 0x01;
    const MSG_TYPE_RESPONSE: u8 = 0x02;

    const CMD_GET_INFO: u8 = 0x01;
    const CMD_SET_POWER: u8 = 0x02;

    const PAYLOAD_INFO: u8 = 0x01;
    const PAYLOAD_POWER_STATE: u8 = 0x02;
    const PAYLOAD_ACK: u8 = 0x03;

    impl Request {
        pub fn encode(&self) -> Vec<u8> {
            let mut buf = Vec::new();
            buf.push(MSG_TYPE_REQUEST);
            buf.extend_from_slice(&self.seq.to_be_bytes());
            match &self.command {
                Command::GetInfo => {
                    buf.push(CMD_GET_INFO);
                }
                Command::SetPower(state) => {
                    buf.push(CMD_SET_POWER);
                    buf.push(if *state { 1 } else { 0 });
                }
            }
            buf
        }
    }

    impl TryFrom<&[u8]> for Response {
        type Error = DecodeError;

        fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
            let mut idx: usize = 0;
            if data.len() < 1 + 4 + 1 {
                return Err(DecodeError::UnexpectedEof);
            }
            let msg_type = data[idx];
            idx += 1;
            if msg_type != MSG_TYPE_RESPONSE {
                return Err(DecodeError::InvalidMessageType(msg_type));
            }
            let seq = u32::from_be_bytes([
                data[idx],
                data[idx + 1],
                data[idx + 2],
                data[idx + 3],
            ]);
            idx += 4;
            let payload_id = data[idx];
            idx += 1;
            let payload = match payload_id {
                PAYLOAD_INFO => {
                    // model length (u8), model bytes, firmware length (u8), firmware bytes
                    if idx + 1 > data.len() {
                        return Err(DecodeError::UnexpectedEof);
                    }
                    let model_len = data[idx] as usize;
                    idx += 1;
                    if idx + model_len + 1 > data.len() {
                        return Err(DecodeError::UnexpectedEof);
                    }
                    let model_bytes = &data[idx..idx + model_len];
                    idx += model_len;
                    let firmware_len = data[idx] as usize;
                    idx += 1;
                    if idx + firmware_len > data.len() {
                        return Err(DecodeError::UnexpectedEof);
                    }
                    let firmware_bytes = &data[idx..idx + firmware_len];
                    idx += firmware_len;
                    let model = String::from_utf8(model_bytes.to_vec())
                        .map_err(DecodeError::Utf8Error)?;
                    let firmware = String::from_utf8(firmware_bytes.to_vec())
                        .map_err(DecodeError::Utf8Error)?;
                    ResponsePayload::Info { model, firmware }
                }
                PAYLOAD_POWER_STATE => {
                    if idx >= data.len() {
                        return Err(DecodeError::UnexpectedEof);
                    }
                    let state = data[idx] != 0;
                    idx += 1;
                    ResponsePayload::PowerState(state)
                }
                PAYLOAD_ACK => ResponsePayload::Ack,
                other => return Err(DecodeError::UnknownPayloadId(other)),
            };
            Ok(Response { seq, payload })
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn encode_get_info() {
            let req = Request {
                seq: 42,
                command: Command::GetInfo,
            };
            let encoded = req.encode();
            assert_eq!(encoded, vec![MSG_TYPE_REQUEST, 0, 0, 0, 42, CMD_GET_INFO]);
        }

        #[test]
        fn encode_set_power() {
            let req = Request {
                seq: 7,
                command: Command::SetPower(true),
            };
            let encoded = req.encode();
            assert_eq!(encoded, vec![MSG_TYPE_REQUEST, 0, 0, 0, 7, CMD_SET_POWER, 1]);
        }

        #[test]
        fn decode_info_response() {
            // seq=5, payload=Info(model="Tapo", firmware="1.2")
            let mut data = vec![
                MSG_TYPE_RESPONSE,
                0, 0, 0, 5,
                PAYLOAD_INFO,
                4, // model len
            ];
            data.extend_from_slice(b"Tapo");
            data.push(3); // firmware len
            data.extend_from_slice(b"1.2");
            let resp = Response::try_from(data.as_slice()).unwrap();
            assert_eq!(
                resp,
                Response {
                    seq: 5,
                    payload: ResponsePayload::Info {
                        model: "Tapo".to_string(),
                        firmware: "1.2".to_string()
                    }
                }
            );
        }

        #[test]
        fn decode_power_state_response() {
            let data = vec![
                MSG_TYPE_RESPONSE,
                0, 0, 0, 9,
                PAYLOAD_POWER_STATE,
                0,
            ];
            let resp = Response::try_from(data.as_slice()).unwrap();
            assert_eq!(
                resp,
                Response {
                    seq: 9,
                    payload: ResponsePayload::PowerState(false)
                }
            );
        }

        #[test]
        fn decode_ack_response() {
            let data = vec![
                MSG_TYPE_RESPONSE,
                0, 0, 0, 255,
                PAYLOAD_ACK,
            ];
            let resp = Response::try_from(data.as_slice()).unwrap();
            assert_eq!(
                resp,
                Response {
                    seq: 255,
                    payload: ResponsePayload::Ack
                }
            );
        }
    }
}