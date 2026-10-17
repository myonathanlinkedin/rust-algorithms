mod core;

use core::Verge;

fn main() {
    // Simple runtime sanity checks.
    let mut v = Verge::new(5);
    v.add(0, 1);
    v.add(1, 2);
    v.add(2, 3);
    assert_eq!(v.range_sum(0, 3), 6);
    assert_eq!(v.get(1), 2);
    println!("All runtime checks passed.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree() {
        let v = Verge::new(0);
        assert_eq!(v.range_sum(0, 0), 0);
    }

    #[test]
    fn test_single_element() {
        let mut v = Verge::new(1);
        v.add(0, 10);
        assert_eq!(v.get(0), 10);
        assert_eq!(v.prefix_sum(1), 10);
    }

    #[test]
    fn test_multiple_updates_and_queries() {
        let mut v = Verge::new(10);
        for i in 0..10 {
            v.add(i, i as i64);
        }
        // Sum of 0..9 = 45
        assert_eq!(v.prefix_sum(10), 45);
        // Sum of 3..7 = 3+4+5+6 = 18
        assert_eq!(v.range_sum(3, 7), 18);
        // Update index 5 by +10
        v.add(5, 10);
        assert_eq!(v.get(5), 5 + 10);
        // Whole range now increased by 10
        assert_eq!(v.range_sum(0, 10), 55);
    }

    #[test]
    fn test_bounds_checking() {
        let mut v = Verge::new(3);
        v.add(2, 5);
        assert_eq!(v.get(2), 5);
        // Out‑of‑bounds should panic.
        let result = std::panic::catch_unwind(|| v.range_sum(0, 4));
        assert!(result.is_err());
    }

    #[test]
    fn test_negative_updates() {
        let mut v = Verge::new(4);
        v.add(0, 5);
        v.add(0, -2);
        assert_eq!(v.get(0), 3);
    }

    #[test]
    fn test_large_values_no_overflow() {
        let mut v = Verge::new(2);
        let big = i64::MAX / 2;
        v.add(0, big);
        v.add(1, big);
        assert_eq!(v.range_sum(0, 2), big + big);
    }
}