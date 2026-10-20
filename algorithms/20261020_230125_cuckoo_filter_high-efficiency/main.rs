mod core;
use core::CuckooFilter;

fn main() {
    // Basic sanity checks executed at runtime.
    let mut filter = CuckooFilter::new(100);
    assert!(filter.insert(&"hello"));
    assert!(filter.contains(&"hello"));
    assert!(filter.delete(&"hello"));
    assert!(!filter.contains(&"hello"));
    // Ensure false positives are rare for a small set.
    let mut false_positives = 0;
    for i in 0..1000 {
        if filter.contains(&i) {
            false_positives += 1;
        }
    }
    // With an empty filter, any positive is a false positive.
    assert!(false_positives <= 5); // very low probability bound
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_insert_lookup_delete() {
        let mut cf = CuckooFilter::new(32);
        assert!(!cf.contains(&42u64));
        assert!(cf.insert(&42u64));
        assert!(cf.contains(&42u64));
        assert!(cf.delete(&42u64));
        assert!(!cf.contains(&42u64));
    }

    #[test]
    fn duplicate_inserts() {
        let mut cf = CuckooFilter::new(64);
        for _ in 0..5 {
            assert!(cf.insert(&"dup"));
        }
        // All duplicates should be reported as present.
        assert!(cf.contains(&"dup"));
        // Deleting once removes only one copy.
        assert!(cf.delete(&"dup"));
        assert!(cf.contains(&"dup"));
    }

    #[test]
    fn fill_until_full() {
        let mut cf = CuckooFilter::new(128);
        let mut inserted = 0;
        for i in 0..1000 {
            if cf.insert(&i) {
                inserted += 1;
            } else {
                break;
            }
        }
        // The filter should eventually reject insertions.
        assert!(inserted > 0);
        assert!(inserted < 1000);
    }

    #[test]
    fn false_positive_rate() {
        let mut cf = CuckooFilter::new(1024);
        for i in 0..500 {
            cf.insert(&i);
        }
        let mut false_pos = 0;
        let trials = 2000;
        for i in 5000..5000 + trials {
            if cf.contains(&i) {
                false_pos += 1;
            }
        }
        let rate = false_pos as f64 / trials as f64;
        // Expected false positive rate for 4‑slot buckets is ~3 %
        assert!(rate < 0.07);
    }
}