mod core;
use core::PNCounter;

fn main() {
    // Simple sanity check executed at runtime.
    let mut a = PNCounter::new(0);
    a.increment(5);
    a.decrement(2);
    assert_eq!(a.value(), 3);
    // Merge with an empty replica – should be a no‑op.
    let b = PNCounter::new(1);
    a.merge(&b);
    assert_eq!(a.value(), 3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_inc_dec() {
        let mut c = PNCounter::new(42);
        assert_eq!(c.value(), 0);
        c.increment(10);
        assert_eq!(c.value(), 10);
        c.decrement(3);
        assert_eq!(c.value(), 7);
        c.increment(0);
        c.decrement(0);
        assert_eq!(c.value(), 7);
    }

    #[test]
    fn merge_commutative() {
        let mut a = PNCounter::new(0);
        let mut b = PNCounter::new(1);

        a.increment(4);
        a.decrement(1); // a = 3

        b.increment(2);
        b.decrement(5); // b = -3

        let mut a_clone = a.clone();
        let mut b_clone = b.clone();

        a.merge(&b);
        b_clone.merge(&a_clone);
        assert_eq!(a.value(), b_clone.value());
        assert_eq!(a.value(), 0);
    }

    #[test]
    fn merge_associative() {
        let mut a = PNCounter::new(0);
        let mut b = PNCounter::new(1);
        let mut c = PNCounter::new(2);

        a.increment(5);
        b.increment(3);
        c.increment(7);

        // (a merge b) merge c
        let mut ab = a.clone();
        ab.merge(&b);
        let mut abc = ab.clone();
        abc.merge(&c);

        // a merge (b merge c)
        let mut bc = b.clone();
        bc.merge(&c);
        let mut a_bc = a.clone();
        a_bc.merge(&bc);

        assert_eq!(abc.value(), a_bc.value());
        assert_eq!(abc.value(), 15);
    }

    #[test]
    fn merge_idempotent() {
        let mut a = PNCounter::new(0);
        a.increment(8);
        a.decrement(2);
        let before = a.clone();
        a.merge(&before);
        assert_eq!(a, before);
    }

    #[test]
    fn concurrent_updates_and_merge() {
        // Simulate three replicas performing independent updates.
        let mut r0 = PNCounter::new(0);
        let mut r1 = PNCounter::new(1);
        let mut r2 = PNCounter::new(2);

        r0.increment(10);
        r1.increment(5);
        r2.decrement(3);

        // Pairwise merges.
        r0.merge(&r1);
        r0.merge(&r2);
        r1.merge(&r2);
        r1.merge(&r0);
        r2.merge(&r0);
        r2.merge(&r1);

        // All replicas must converge to the same value.
        let expected = 10 + 5 - 3;
        assert_eq!(r0.value(), expected);
        assert_eq!(r1.value(), expected);
        assert_eq!(r2.value(), expected);
    }

    #[test]
    #[should_panic(expected = "PNCounter increment overflow")]
    fn overflow_increment() {
        let mut c = PNCounter::new(0);
        c.increment(u64::MAX);
        // This second increment would overflow.
        c.increment(1);
    }

    #[test]
    #[should_panic(expected = "PNCounter decrement overflow")]
    fn overflow_decrement() {
        let mut c = PNCounter::new(0);
        c.decrement(u64::MAX);
        c.decrement(1);
    }
}