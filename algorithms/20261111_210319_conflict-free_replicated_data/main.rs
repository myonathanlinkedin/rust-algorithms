mod core;

use core::PNCounter;

fn main() {
    // Basic sanity checks executed at runtime.
    let mut a = PNCounter::new(0);
    let mut b = PNCounter::new(1);

    a.inc(5);
    b.inc(3);
    b.dec(2);

    // Before merge values differ.
    assert_eq!(a.value(), 5);
    assert_eq!(b.value(), 1);

    // Merge bi‑directionally.
    a.merge(&b);
    b.merge(&a);

    assert_eq!(a.value(), 6);
    assert_eq!(b.value(), 6);

    // Further operations after merge.
    a.dec(1);
    b.inc(4);
    a.merge(&b);

    assert_eq!(a.value(), 9);
    // Verify internal per‑replica state.
    assert_eq!(a.p.get(&0), Some(&4)); // 5 inc -1 dec
    assert_eq!(a.n.get(&0), Some(&1));
    assert_eq!(a.p.get(&1), Some(&7)); // 3 inc +4 inc
    assert_eq!(a.n.get(&1), Some(&2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let mut c = PNCounter::new(42);
        assert_eq!(c.value(), 0);
        c.inc(10);
        assert_eq!(c.value(), 10);
        c.dec(3);
        assert_eq!(c.value(), 7);
        c.inc(0); // defaults to 1
        assert_eq!(c.value(), 8);
        c.dec(0); // defaults to 1
        assert_eq!(c.value(), 7);
    }

    #[test]
    fn test_merge_commutativity() {
        let mut x = PNCounter::new(0);
        let mut y = PNCounter::new(1);
        x.inc(4);
        y.dec(2);
        let mut x_clone = x.clone();
        let mut y_clone = y.clone();

        x.merge(&y);
        y.merge(&x_clone);
        assert_eq!(x.value(), y.value());

        // Merge in opposite order
        x_clone.merge(&y_clone);
        y_clone.merge(&x);
        assert_eq!(x_clone.value(), y_clone.value());
        assert_eq!(x.value(), x_clone.value());
    }

    #[test]
    fn test_merge_idempotence() {
        let mut a = PNCounter::new(0);
        a.inc(5);
        a.dec(2);
        let before = a.clone();
        a.merge(&before);
        assert_eq!(a, before);
    }

    #[test]
    fn test_merge_associativity() {
        let mut a = PNCounter::new(0);
        let mut b = PNCounter::new(1);
        let mut c = PNCounter::new(2);

        a.inc(3);
        b.inc(5);
        c.dec(2);

        // (a merge b) merge c
        let mut ab = a.clone();
        ab.merge(&b);
        let mut abc1 = ab.clone();
        abc1.merge(&c);

        // a merge (b merge c)
        let mut bc = b.clone();
        bc.merge(&c);
        let mut abc2 = a.clone();
        abc2.merge(&bc);

        assert_eq!(abc1, abc2);
        assert_eq!(abc1.value(), 6); // 3+5-2
    }

    #[test]
    fn test_overflow_saturation() {
        let mut counter = PNCounter::new(0);
        counter.inc(u64::MAX);
        counter.inc(1); // should saturate at u64::MAX
        assert_eq!(*counter.p.get(&0).unwrap(), u64::MAX);
        counter.dec(u64::MAX);
        counter.dec(1); // saturate at u64::MAX
        assert_eq!(*counter.n.get(&0).unwrap(), u64::MAX);
        // Value should be 0 because both sides saturated equally.
        assert_eq!(counter.value(), 0);
    }
}