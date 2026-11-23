mod core;
use core::PNCounter;

fn basic_asserts() {
    // Create two replicas
    let mut a = PNCounter::new("A");
    let mut b = PNCounter::new("B");

    // Perform operations
    a.increment(5);
    a.decrement(2);
    b.increment(3);
    b.decrement(1);
    b.increment(4); // total increments on B = 7

    // Verify local values
    assert_eq!(a.value(), 3); // 5 - 2
    assert_eq!(b.value(), 6); // 7 - 1

    // Merge B into A and check convergence
    a.merge(&b);
    b.merge(&a);
    assert_eq!(a.value(), 9); // (5+7) - (2+1) = 9
    assert_eq!(b.value(), 9);
    assert_eq!(a, b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment_decrement() {
        let mut c = PNCounter::new("C");
        assert_eq!(c.value(), 0);
        c.increment(10);
        assert_eq!(c.value(), 10);
        c.decrement(4);
        assert_eq!(c.value(), 6);
        // Re‑apply same increments/decrements should be cumulative
        c.increment(5);
        c.decrement(1);
        assert_eq!(c.value(), 10);
    }

    #[test]
    fn test_merge_commutative() {
        let mut x = PNCounter::new("X");
        let mut y = PNCounter::new("Y");
        x.increment(3);
        y.increment(5);
        let mut x_clone = x.clone();
        let mut y_clone = y.clone();

        x.merge(&y);
        y.merge(&x_clone);
        assert_eq!(x, y);
        assert_eq!(x.value(), 8);
        assert_eq!(y.value(), 8);
        // Ensure merging in opposite order yields same state
        x_clone.merge(&y_clone);
        assert_eq!(x, x_clone);
    }

    #[test]
    fn test_merge_idempotent() {
        let mut p = PNCounter::new("P");
        p.increment(2);
        p.decrement(1);
        let before = p.clone();
        p.merge(&before);
        assert_eq!(p, before);
    }

    #[test]
    fn test_convergence_multiple_replicas() {
        // Three replicas performing independent operations
        let mut r1 = PNCounter::new("R1");
        let mut r2 = PNCounter::new("R2");
        let mut r3 = PNCounter::new("R3");

        r1.increment(4);
        r2.increment(7);
        r3.decrement(3);
        r1.decrement(1);
        r2.increment(2);
        r3.increment(5);

        // Pairwise merges
        r1.merge(&r2);
        r2.merge(&r3);
        r3.merge(&r1);

        // Full mesh merges
        r1.merge(&r3);
        r2.merge(&r1);
        r3.merge(&r2);

        let expected = (4 + 7 + 2 + 5) as i64 - (1 + 3) as i64; // 18 - 4 = 14
        assert_eq!(r1.value(), expected);
        assert_eq!(r2.value(), expected);
        assert_eq!(r3.value(), expected);
        assert_eq!(r1, r2);
        assert_eq!(r2, r3);
    }
}

fn main() {
    // Run a quick sanity check at program start.
    basic_asserts();
    // If the program reaches this point without panicking, all assertions passed.
    println!("All PN‑Counter sanity checks passed.");
}