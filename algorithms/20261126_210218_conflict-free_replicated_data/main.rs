mod core;

use core::PNCounter;

fn main() {
    let mut counter_a = PNCounter::new();
    counter_a.inc(1);
    counter_a.inc(1);
    counter_a.dec(2);
    assert_eq!(counter_a.value(), 1);

    let mut counter_b = PNCounter::new();
    counter_b.inc(2);
    counter_b.dec(1);
    assert_eq!(counter_b.value(), 0);

    counter_a.merge(&counter_b);
    assert_eq!(counter_a.value(), 1);

    let mut counter_c = counter_a.clone();
    counter_a.merge(&counter_c);
    assert_eq!(counter_a.value(), counter_c.value());

    let mut counter_d = PNCounter::new();
    counter_d.inc(3);
    counter_d.dec(3);
    let mut counter_e = PNCounter::new();
    counter_e.inc(3);
    counter_e.dec(3);
    counter_d.merge(&counter_e);
    counter_e.merge(&counter_d);
    assert_eq!(counter_d.value(), counter_e.value());

    let mut counter_f = PNCounter::new();
    counter_f.inc(4);
    let mut counter_g = PNCounter::new();
    counter_g.dec(4);
    let mut counter_h = PNCounter::new();
    counter_h.inc(4);
    counter_f.merge(&counter_g);
    counter_f.merge(&counter_h);
    let mut counter_i = counter_f.clone();
    counter_g.merge(&counter_h);
    counter_i.merge(&counter_g);
    assert_eq!(counter_f.value(), counter_i.value());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inc_dec_value() {
        let mut counter = PNCounter::new();
        counter.inc(0);
        counter.inc(0);
        counter.dec(1);
        assert_eq!(counter.value(), 1);
    }

    #[test]
    fn test_merge_max() {
        let mut a = PNCounter::new();
        a.inc(0);
        a.inc(0);
        a.dec(1);

        let mut b = PNCounter::new();
        b.inc(0);
        b.dec(1);
        b.dec(1);

        a.merge(&b);
        assert_eq!(a.value(), 0);
    }

    #[test]
    fn test_idempotent_merge() {
        let mut a = PNCounter::new();
        a.inc(0);
        let mut b = a.clone();
        a.merge(&b);
        a.merge(&b);
        assert_eq!(a.value(), 1);
    }

    #[test]
    fn test_commutative_merge() {
        let mut a = PNCounter::new();
        a.inc(0);
        a.dec(1);

        let mut b = PNCounter::new();
        b.dec(1);
        b.inc(0);

        let mut a_copy = a.clone();
        let mut b_copy = b.clone();

        a_copy.merge(&b_copy);
        b_copy.merge(&a_copy);

        assert_eq!(a_copy.value(), b_copy.value());
    }

    #[test]
    fn test_associative_merge() {
        let mut a = PNCounter::new();
        a.inc(0);

        let mut b = PNCounter::new();
        b.dec(0);

        let mut c = PNCounter::new();
        c.inc(0);

        let mut ab = a.clone();
        ab.merge(&b);
        ab.merge(&c);

        let mut bc = b.clone();
        bc.merge(&c);
        let mut abc = a.clone();
        abc.merge(&bc);

        assert_eq!(ab.value(), abc.value());
    }
}