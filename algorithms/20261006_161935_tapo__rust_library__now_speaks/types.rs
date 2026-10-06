pub mod types {
    #[derive(Debug, Clone, PartialEq)]
    pub enum Command {
        GetStatus,
        SetPower(bool),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Request {
        pub command: Command,

    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Response {
        pub success: bool,
        pub payload: ResponsePayload,

    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum ResponsePayload {
        Status(bool), // power on/off
        Ack,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Message {
        Request(Request),
        Response(Response),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum DecodeError {
        InvalidStartByte,
        LengthMismatch,
        ChecksumMismatch,
        UnknownCommand(u8),
        UnexpectedEof,
    }
}

// Re-export for easier access
pub use types::*;