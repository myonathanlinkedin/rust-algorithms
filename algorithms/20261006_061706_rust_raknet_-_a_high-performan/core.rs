#![allow(dead_code)]

use std::collections::{HashMap, HashSet, VecDeque};

/// Sequence number with wrap‑around aware comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SeqNum(pub u32);

impl SeqNum {
    /// Returns the next sequence number, wrapping at 2³².
    pub fn next(self) -> Self {
        SeqNum(self.0.wrapping_add(1))
    }

    /// Returns true if `self` is less than `other` in the circular space.
    /// Implements the classic RakNet comparison: distance < 2³¹.
    pub fn less_than(self, other: Self) -> bool {
        let diff = other.0.wrapping_sub(self.0);
        diff != 0 && diff < (1u32 << 31)
    }
}

/// Header used by RakNet packets (simplified).
#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    pub seq: SeqNum,
    pub ack: SeqNum,
    pub ack_bitfield: u32,

}

impl Header {
    pub fn encode(&self) -> [u8; 12] {
        let mut out = [0u8; 12];
        out[0..4].copy_from_slice(&self.seq.0.to_be_bytes());
        out[4..8].copy_from_slice(&self.ack.0.to_be_bytes());
        out[8..12].copy_from_slice(&self.ack_bitfield.to_be_bytes());
        out
    }

    pub fn decode(buf: &[u8]) -> Option<Self> {
        if buf.len() < 12 {
            return None;
        }
        let seq = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
        let ack = u32::from_be_bytes([buf[4], buf[5], buf[6], buf[7]]);
        let ack_bitfield = u32::from_be_bytes([buf[8], buf[9], buf[10], buf[11]]);
        Some(Header {
            seq: SeqNum(seq),
            ack: SeqNum(ack),
            ack_bitfield,
        })
    }
}

/// Full RakNet packet (header + payload).
#[derive(Debug, Clone, PartialEq)]
pub struct RakPacket {
    pub header: Header,
    pub payload: Vec<u8>,

}

impl RakPacket {
    pub fn new(seq: SeqNum, ack: SeqNum, ack_bitfield: u32, payload: Vec<u8>) -> Self {
        RakPacket {
            header: Header {
                seq,
                ack,
                ack_bitfield,
            },
            payload,
        }
    }

    /// Serialises the packet to a byte vector.
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(12 + self.payload.len());
        out.extend_from_slice(&self.header.encode());
        out.extend_from_slice(&self.payload);
        out
    }

    /// Deserialises a packet from a byte slice.
    pub fn decode(buf: &[u8]) -> Option<Self> {
        let header = Header::decode(buf)?;
        let payload = buf[12..].to_vec();
        Some(RakPacket { header, payload })
    }
}

/// Manages reliable transmission state.
#[derive(Debug, Clone, PartialEq)]
pub struct ReliableQueue<T> {
    /// Packets waiting to be sent (ordered by insertion).
    pub send_queue: VecDeque<(SeqNum, T)>,
    /// Packets that have been sent but not yet acked.
    pub sent: HashMap<u32, T>,
    /// Set of acked sequence numbers.
    pub acked: HashSet<u32>,
    /// Next sequence number to assign.
    pub next_seq: SeqNum,

}

impl<T: Clone> ReliableQueue<T> {
    pub fn new() -> Self {
        ReliableQueue {
            send_queue: VecDeque::new(),
            sent: HashMap::new(),
            acked: HashSet::new(),
            next_seq: SeqNum(0),
        }
    }

    /// Queue a payload for reliable transmission.
    pub fn enqueue(&mut self, payload: T) {
        let seq = self.next_seq;
        self.next_seq = self.next_seq.next();
        self.send_queue.push_back((seq, payload));
    }

    /// Retrieve the next packet ready to be transmitted.
    /// Moves it from the send queue to the sent map.
    pub fn pop_for_send(&mut self) -> Option<(SeqNum, T)> {
        if let Some((seq, payload)) = self.send_queue.pop_front() {
            self.sent.insert(seq.0, payload.clone());
            Some((seq, payload))
        } else {
            None
        }
    }

    /// Process an incoming ACK for `seq`.
    pub fn acknowledge(&mut self, seq: SeqNum) {
        if self.sent.remove(&seq.0).is_some() {
            self.acked.insert(seq.0);
        }
    }

    /// Process an ACK bitfield relative to `ack_base`.
    /// Bits set to 1 indicate that `ack_base - i - 1` is also acked.
    pub fn acknowledge_bitfield(&mut self, ack_base: SeqNum, bitfield: u32) {
        // Ack the base sequence.
        self.acknowledge(ack_base);
        // Iterate over bits.
        for i in 0..32 {
            if (bitfield >> i) & 1 == 1 {
                let seq_num = ack_base.0.wrapping_sub(i as u32 + 1);
                self.acknowledge(SeqNum(seq_num));
            }
        }
    }

    /// Returns true if all queued packets have been acked.
    pub fn is_empty(&self) -> bool {
        self.send_queue.is_empty() && self.sent.is_empty()
    }
}

/// Helper to generate ACK bitfields for a set of received sequence numbers.
pub fn generate_ack_bitfield(received: &HashSet<u32>, latest: SeqNum) -> u32 {
    let mut bits = 0u32;
    for i in 0..32 {
        let seq = latest.0.wrapping_sub(i as u32 + 1);
        if received.contains(&seq) {
            bits |= 1 << i;
        }
    }
    bits
}

#[cfg(test)]
fn main() {}
