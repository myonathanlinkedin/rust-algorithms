use crate::types::HttpRequest;

/// Build a raw HTTP/1.1 request from an `HttpRequest`.
pub fn build_http_request(req: &HttpRequest) -> Vec<u8> {
    let mut out = Vec::new();

    out.extend_from_slice(req.method.as_bytes());
    out.extend_from_slice(b" ");
    out.extend_from_slice(req.url.as_bytes());
    out.extend_from_slice(b" HTTP/1.1\r\n");

    for (name, value) in &req.headers {
        out.extend_from_slice(name.as_bytes());
        out.extend_from_slice(b": ");
        out.extend_from_slice(value.as_bytes());
        out.extend_from_slice(b"\r\n");
    }

    out.extend_from_slice(b"\r\n");
    out.extend_from_slice(&req.body);
    out
}

/// Compute a simple checksum by summing all bytes (wrapping addition).
pub fn checksum(data: &[u8]) -> u32 {
    let mut sum: u32 = 0;
    for idx in 0..data.len() {
        sum = sum.wrapping_add(data[idx] as u32);
    }
    sum
}