mod types;
mod engine;

use types::HttpRequest;
use engine::{build_http_request, checksum};

fn test_checksum() {
    let data = b"The quick brown fox jumps over the lazy dog";
    let expected = data.iter().fold(0u32, |acc, &b| acc.wrapping_add(b as u32));
    assert_eq!(checksum(data), expected, "checksum should match manual sum");
}

fn test_build_http_request() {
    let req = HttpRequest {
        method: "POST".into(),
        url: "/submit".into(),
        headers: vec![
            ("Host".into(), "example.org".into()),
            ("Content-Type".into(), "application/json".into()),
            ("Content-Length".into(), "18".into()),
        ],
        body: b"{\"key\":\"value\"}".to_vec(),
    };

    let raw = build_http_request(&req);
    let expected = b"POST /submit HTTP/1.1\r\n\
Host: example.org\r\n\
Content-Type: application/json\r\n\
Content-Length: 18\r\n\
\r\n\
{\"key\":\"value\"}"
        .to_vec();

    assert_eq!(raw, expected, "generated HTTP request does not match expected output");
}

fn main() {
    // Run all assertions; panic on failure.
    test_checksum();
    test_build_http_request();

    // Demonstration output
    let demo_req = HttpRequest {
        method: "GET".into(),
        url: "/".into(),
        headers: vec![("User-Agent".into(), "RustDemo/1.0".into())],
        body: Vec::new(),
    };
    let demo_raw = build_http_request(&demo_req);
    println!("{}", String::from_utf8_lossy(&demo_raw));
}