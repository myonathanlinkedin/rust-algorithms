mod core;

use core::{ClockRelation, VectorClock};

fn main() {
    // Basic runtime sanity checks.
    let mut vc = VectorClock::new(3);
    vc.increment(0);
    vc.increment(2);
    assert_eq!(vc.get(0), 1);
    assert_eq!(vc.get(1), 0);
    assert_eq!(vc.get(2), 1);

    let mut other = VectorClock::new(3);
    other.increment(1);
    other.increment(2);
    other.increment(2); // counter[2] == 2

    // Merge `other` into a clone of `vc`.
    let mut merged = vc.clone();
    merged.merge(&other);
    assert_eq!(merged.get(0), 1);
    assert_eq!(merged.get(1), 1);
    assert_eq!(merged.get(2), 2);

    // Comparison outcomes.
    assert_eq!(vc.compare(&other), ClockRelation::Concurrent);
    assert_eq!(vc.compare(&merged), ClockRelation::Less);
    assert_eq!(other.compare(&merged), ClockRelation::Less);
    assert_eq!(merged.compare(&merged), ClockRelation::Equal);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment_and_get() {
        let mut vc = VectorClock::new(2);
        vc.increment(0);
        vc.increment(0);
        vc.increment(1);
        assert_eq!(vc.get(0), 2);
        assert_eq!(vc.get(1), 1);
    }

    #[test]
    fn test_merge_maximum() {
        let mut a = VectorClock::new(3);
        a.set(0, 3);
        a.set(1, 1);
        a.set(2, 4);
        let mut b = VectorClock::new(3);
        b.set(0, 2);
        b.set(1, 5);
        b.set(2, 1);
        a.merge(&b);
        assert_eq!(a.get(0), 3);
        assert_eq!(a.get(1), 5);
        assert_eq!(a.get(2), 4);
    }

    #[test]
    fn test_compare_equal() {
        let mut a = VectorClock::new(2);
        a.increment(0);
        a.increment(1);
        let b = a.clone();
        assert_eq!(a.compare(&b), ClockRelation::Equal);
    }

    #[test]
    fn test_compare_less_and_greater() {
        let mut a = VectorClock::new(2);
        a.set(0, 1);
        a.set(1, 2);
        let mut b = VectorClock::new(2);
        b.set(0, 1);
        b.set(1, 3);
        assert_eq!(a.compare(&b), ClockRelation::Less);
        assert_eq!(b.compare(&a), ClockRelation::Greater);
    }

    #[test]
    fn test_compare_concurrent() {
        let mut a = VectorClock::new(2);
        a.set(0, 2);
        a.set(1, 1);
        let mut b = VectorClock::new(2);
        b.set(0, 1);
        b.set(1, 2);
        assert_eq!(a.compare(&b), ClockRelation::Concurrent);
        assert_eq!(b.compare(&a), ClockRelation::Concurrent);
    }

    #[test]
    #[should_panic(expected = "pid out of bounds")]
    fn test_invalid_pid() {
        let mut vc = VectorClock::new(1);
        vc.increment(2);
    }

    #[test]
    #[should_panic(expected = "size mismatch")]
    fn test_merge_size_mismatch() {
        let mut a = VectorClock::new(2);
        let b = VectorClock::new(3);
        a.merge(&b);
    }

    #[test]
    #[should_panic(expected = "size mismatch")]
    fn test_compare_size_mismatch() {
        let a = VectorClock::new(2);
        let b = VectorClock::new(3);
        a.compare(&b);
    }
}