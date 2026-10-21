mod core;
use core::DSU;

fn main() {
    // Basic sanity checks.
    let mut dsu = DSU::new(5);
    assert_eq!(dsu.size(0), 1);
    assert!(!dsu.connected(0, 1));
    assert!(dsu.union(0, 1));
    assert!(dsu.connected(0, 1));
    assert_eq!(dsu.size(0), 2);
    assert_eq!(dsu.size(1), 2);
    // Redundant union returns false.
    assert!(!dsu.union(0, 1));
    // More unions.
    dsu.union(2, 3);
    dsu.union(3, 4);
    assert!(dsu.connected(2, 4));
    assert_eq!(dsu.size(2), 3);
    // Merge two groups.
    dsu.union(1, 4);
    assert!(dsu.connected(0, 4));
    assert_eq!(dsu.size(0), 5);
    // Single-element DSU edge case.
    let mut single = DSU::new(1);
    assert_eq!(single.size(0), 1);
    assert!(!single.union(0, 0));
    println!("All DSU assertions passed.");
}

#[cfg(test)]
mod tests {
    use super::core::DSU;

    #[test]
    fn test_basic_operations() {
        let mut dsu = DSU::new(10);
        for i in 0..10 {
            assert_eq!(dsu.size(i), 1);
        }
        assert!(dsu.union(0, 1));
        assert!(dsu.union(2, 3));
        assert!(!dsu.connected(0, 2));
        dsu.union(1, 2);
        assert!(dsu.connected(0, 3));
        assert_eq!(dsu.size(0), 4);
    }

    #[test]
    fn test_redundant_union() {
        let mut dsu = DSU::new(3);
        assert!(dsu.union(0, 1));
        assert!(!dsu.union(0, 1));
        assert!(!dsu.union(1, 0));
    }

    #[test]
    fn test_large_random() {
        let n = 1000;
        let mut dsu = DSU::new(n);
        // Union sequential pairs.
        for i in (0..n).step_by(2) {
            if i + 1 < n {
                dsu.union(i, i + 1);
            }
        }
        // Verify component sizes.
        for i in 0..n {
            let expected = if i % 2 == 0 && i + 1 < n { 2 } else if i % 2 == 1 { 2 } else { 1 };
            assert_eq!(dsu.size(i), expected);
        }
    }
}