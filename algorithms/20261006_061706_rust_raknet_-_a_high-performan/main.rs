mod core;

use core::{
    generate_ack_bitfield, Header, ReliableQueue, RakPacket, SeqNum,
};
use std::collections::HashSet;

fn test_sequence_wrapping() {
    let a = SeqNum(u32::MAX);
    let b = a.next();
    assert_eq!(b.0, 0);
    assert!(a.less_than(b));
    assert!(!b.less_than(a));
}

fn test_header_encode_decode() {
    let hdr = Header {
        seq: SeqNum(12345),
        ack: SeqNum(54321),
        ack_bitfield: 0xA5A5A5A5,
    };
    let bytes = hdr.encode();
    let decoded = Header::decode(&bytes).unwrap();
    assert_eq!(hdr, decoded);
}

fn test_packet_encode_decode() {
    let payload = b"hello raknet".to_vec();
    let pkt = RakPacket::new(SeqNum(1), SeqNum(0), 0, payload.clone());
    let bytes = pkt.encode();
    let decoded = RakPacket::decode(&bytes).unwrap();
    assert_eq!(pkt, decoded);
    assert_eq!(decoded.payload, payload);
}

fn test_reliable_queue_basic() {
    let mut rq = ReliableQueue::new();
    rq.enqueue(b"msg1".to_vec());
    rq.enqueue(b"msg2".to_vec());

    // Send first packet.
    let (seq1, payload1) = rq.pop_for_send().unwrap();
    assert_eq!(seq1.0, 0);
    assert_eq!(payload1, b"msg1".to_vec());

    // Send second packet.
    let (seq2, payload2) = rq.pop_for_send().unwrap();
    assert_eq!(seq2.0, 1);
    assert_eq!(payload2, b"msg2".to_vec());

    // Ack first packet.
    rq.acknowledge(seq1);
    assert!(!rq.is_empty());

    // Ack second via bitfield (base = seq2, bit 0 set for seq1).
    let mut received = HashSet::new();
    received.insert(seq1.0);
    let bitfield = generate_ack_bitfield(&received, seq2);
    rq.acknowledge_bitfield(seq2, bitfield);
    assert!(rq.is_empty());
}

fn test_ack_bitfield_generation() {
    let mut received = HashSet::new();
    // Simulate receiving seq 100, 98, 97 (missing 99).
    received.insert(100);
    received.insert(98);
    received.insert(97);
    let latest = SeqNum(100);
    let bits = generate_ack_bitfield(&received, latest);
    // Bit 0 corresponds to 99 (missing), bit1 -> 98, bit2 ->97
    assert_eq!(bits, 0b110);
}

fn main() {
    test_sequence_wrapping();
    test_header_encode_decode();
    test_packet_encode_decode();
    test_reliable_queue_basic();
    test_ack_bitfield_generation();
    println!("All RakNet core tests passed.");
}
