mod core;
use core::{DNSRecord, DNSZone, RecordType, Resolver};

fn main() {
    // Simple sanity checks executed at runtime
    let mut zone = DNSZone::new();
    zone.insert(DNSRecord {
        name: "example.com".to_string(),
        rtype: RecordType::A,
        value: "93.184.216.34".to_string(),
    });
    zone.insert(DNSRecord {
        name: "www.example.com".to_string(),
        rtype: RecordType::CNAME,
        value: "example.com".to_string(),
    });

    let resolver = Resolver::new(&zone);
    assert_eq!(
        resolver.resolve("example.com"),
        Some("93.184.216.34".to_string())
    );
    assert_eq!(
        resolver.resolve("www.example.com"),
        Some("93.184.216.34".to_string())
    );
    assert_eq!(resolver.resolve("nonexistent.com"), None);
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_a_record() {
        let mut zone = DNSZone::new();
        zone.insert(DNSRecord {
            name: "a.test".to_string(),
            rtype: RecordType::A,
            value: "1.2.3.4".to_string(),
        });
        let resolver = Resolver::new(&zone);
        assert_eq!(resolver.resolve("a.test"), Some("1.2.3.4".to_string()));
    }

    #[test]
    fn cname_chain_resolution() {
        let mut zone = DNSZone::new();
        zone.insert(DNSRecord {
            name: "c1".to_string(),
            rtype: RecordType::CNAME,
            value: "c2".to_string(),
        });
        zone.insert(DNSRecord {
            name: "c2".to_string(),
            rtype: RecordType::CNAME,
            value: "c3".to_string(),
        });
        zone.insert(DNSRecord {
            name: "c3".to_string(),
            rtype: RecordType::A,
            value: "5.6.7.8".to_string(),
        });
        let resolver = Resolver::new(&zone);
        assert_eq!(resolver.resolve("c1"), Some("5.6.7.8".to_string()));
    }

    #[test]
    fn loop_detection() {
        let mut zone = DNSZone::new();
        zone.insert(DNSRecord {
            name: "loop1".to_string(),
            rtype: RecordType::CNAME,
            value: "loop2".to_string(),
        });
        zone.insert(DNSRecord {
            name: "loop2".to_string(),
            rtype: RecordType::CNAME,
            value: "loop1".to_string(),
        });
        let resolver = Resolver::new(&zone);
        assert_eq!(resolver.resolve("loop1"), None);
    }

    #[test]
    fn max_depth_enforcement() {
        let mut zone = DNSZone::new();
        // Build a chain longer than the default max_depth (10)
        let mut prev = "start".to_string();
        for i in 0..12 {
            let next = format!("node{}", i);
            zone.insert(DNSRecord {
                name: prev.clone(),
                rtype: RecordType::CNAME,
                value: next.clone(),
            });
            prev = next;
        }
        // Final node with an A record
        zone.insert(DNSRecord {
            name: prev.clone(),
            rtype: RecordType::A,
            value: "9.9.9.9".to_string(),
        });
        let resolver = Resolver::new(&zone);
        // Should fail because depth exceeds limit
        assert_eq!(resolver.resolve("start"), None);
    }

    #[test]
    fn mixed_records_preference() {
        let mut zone = DNSZone::new();
        // Both A and CNAME for same name; A should win per implementation
        zone.insert(DNSRecord {
            name: "mixed".to_string(),
            rtype: RecordType::CNAME,
            value: "somewhere".to_string(),
        });
        zone.insert(DNSRecord {
            name: "mixed".to_string(),
            rtype: RecordType::A,
            value: "10.0.0.1".to_string(),
        });
        zone.insert(DNSRecord {
            name: "somewhere".to_string(),
            rtype: RecordType::A,
            value: "10.0.0.2".to_string(),
        });
        let resolver = Resolver::new(&zone);
        // A record is returned directly, not the CNAME target
        assert_eq!(resolver.resolve("mixed"), Some("10.0.0.1".to_string()));
    }
}