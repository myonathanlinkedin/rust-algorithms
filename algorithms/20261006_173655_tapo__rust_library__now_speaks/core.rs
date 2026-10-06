pub mod tpap {
    #[derive(Debug, Clone, PartialEq)]
    pub struct TpapMessage {
        pub id: u32,
        pub method: String,
        /// JSON payload string (already serialized, e.g. "{}" or "{\"state\":\"on\"}")
        pub payload: String,

    }

    impl TpapMessage {
        /// Encode the message into the TPAP JSON wire format.
        /// Example: {"id":1,"method":"get_device_info","payload":{}}
        pub fn encode(&self) -> String {
            // The payload is assumed to be a valid JSON fragment without surrounding quotes.
            format!(
                "{{\"id\":{},\"method\":\"{}\",\"payload\":{}}}",
                self.id,
                self.method,
                self.payload
            )
        }

        /// Decode a TPAP wire format string into a `TpapMessage`.
        /// The parser is deliberately simple and expects the exact format produced by `encode`.
        pub fn decode(s: &str) -> Result<Self, &'static str> {
            // Trim surrounding whitespace.
            let s = s.trim();

            // Must start with '{' and end with '}'
            if !s.starts_with('{') || !s.ends_with('}') {
                return Err("Message must start with '{' and end with '}'");
            }

            // Remove outer braces.
            let inner = &s[1..s.len() - 1];

            // Split by commas that are not inside quotes.
            let mut parts: Vec<&str> = Vec::new();
            let mut start = 0usize;
            let mut in_str = false;
            for (i, c) in inner.char_indices() {
                match c {
                    '"' => in_str = !in_str,
                    ',' if !in_str => {
                        parts.push(&inner[start..i]);
                        start = i + 1;
                    }
                    _ => {}
                }
            }
            parts.push(&inner[start..]);

            // Helper to extract key/value.
            fn get_value<'a>(parts: &'a [ &str ], key: &str) -> Option<&'a str> {
                for part in parts {
                    let kv: Vec<&str> = part.splitn(2, ':').collect();
                    if kv.len() != 2 {
                        continue;
                    }
                    let k = kv[0].trim().trim_matches('"');
                    if k == key {
                        return Some(kv[1].trim());
                    }
                }
                None
            }

            // Extract id.
            let id_str = get_value(&parts, "id").ok_or("Missing id")?;
            let id: u32 = id_str.parse().map_err(|_| "Invalid id")?;

            // Extract method.
            let method_raw = get_value(&parts, "method").ok_or("Missing method")?;
            // method is a quoted string.
            if !method_raw.starts_with('"') || !method_raw.ends_with('"') {
                return Err("Method must be a quoted string");
            }
            let method = method_raw[1..method_raw.len() - 1].to_string();

            // Extract payload.
            let payload_raw = get_value(&parts, "payload").ok_or("Missing payload")?;
            // Payload may be any JSON fragment; keep as is.
            let payload = payload_raw.to_string();

            Ok(TpapMessage { id, method, payload })
        }
    }

    #[cfg(test)]
    mod tests {
        use super::TpapMessage;

        #[test]
        fn encode_decode_roundtrip() {
            let msg = TpapMessage {
                id: 42,
                method: "set_state".to_string(),
                payload: "{\"state\":\"on\"}".to_string(),
            };
            let encoded = msg.encode();
            let decoded = TpapMessage::decode(&encoded).expect("decode should succeed");
            assert_eq!(msg, decoded);
        }

        #[test]
        fn encode_simple_payload() {
            let msg = TpapMessage {
                id: 1,
                method: "get_device_info".to_string(),
                payload: "{}".to_string(),
            };
            let expected = r#"{"id":1,"method":"get_device_info","payload":{}}"#;
            assert_eq!(msg.encode(), expected);
        }

        #[test]
        fn decode_malformed_missing_brace() {
            let bad = r#" "id":1,"method":"foo","payload":{} "#;
            assert!(TpapMessage::decode(bad).is_err());
        }

        #[test]
        fn decode_invalid_id() {
            let bad = r#"{"id":"abc","method":"foo","payload":{}}"#;
            assert!(TpapMessage::decode(bad).is_err());
        }

        #[test]
        fn decode_missing_fields() {
            let bad = r#"{"id":1,"payload":{}}"#;
            assert!(TpapMessage::decode(bad).is_err());
        }
    }
}