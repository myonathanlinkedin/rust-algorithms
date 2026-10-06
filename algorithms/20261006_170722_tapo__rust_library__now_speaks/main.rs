mod core;
use core::tpap::*;

fn main() {
    // Test encoding of a GetInfo request
    let req_get = Request {
        seq: 1,
        command: Command::GetInfo,
    };
    let enc_get = req_get.encode();
    assert_eq!(enc_get, vec![0x01, 0, 0, 0, 1, 0x01]);

    // Test encoding of a SetPower request (off)
    let req_set = Request {
        seq: 2,
        command: Command::SetPower(false),
    };
    let enc_set = req_set.encode();
    assert_eq!(enc_set, vec![0x01, 0, 0, 0, 2, 0x02, 0]);

    // Simulate a response for GetInfo
    let mut resp_bytes = vec![
        0x02, // response type
        0, 0, 0, 1, // seq
        0x01, // payload info
        5, // model len
    ];
    resp_bytes.extend_from_slice(b"Model");
    resp_bytes.push(4); // firmware len
    resp_bytes.extend_from_slice(b"v1.0");
    let resp = Response::try_from(resp_bytes.as_slice()).expect("decode info");
    match resp.payload {
        ResponsePayload::Info { model, firmware } => {
            assert_eq!(model, "Model");
            assert_eq!(firmware, "v1.0");
        }
        _ => panic!("unexpected payload"),
    }

    // Simulate a PowerState response (on)
    let resp_power = vec![
        0x02,
        0, 0, 0, 2,
        0x02,
        1,
    ];
    let resp = Response::try_from(resp_power.as_slice()).expect("decode power");
    assert_eq!(
        resp,
        Response {
            seq: 2,
            payload: ResponsePayload::PowerState(true)
        }
    );

    // Simulate an Ack response
    let resp_ack = vec![
        0x02,
        0, 0, 0, 3,
        0x03,
    ];
    let resp = Response::try_from(resp_ack.as_slice()).expect("decode ack");
    assert_eq!(
        resp,
        Response {
            seq: 3,
            payload: ResponsePayload::Ack
        }
    );

    // All assertions passed
    println!("All TPAP protocol unit tests passed.");
}