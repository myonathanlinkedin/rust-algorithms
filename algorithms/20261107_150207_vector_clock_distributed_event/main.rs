mod types;
mod engine;

use crate::types::{VectorClock, ClockRelation};

fn main() {
    // Basic sanity checks
    let mut vc_a = VectorClock::new(3);
    let mut vc_b = VectorClock::new(3);

    // Initially both clocks are equal
    assert_eq!(vc_a.compare(&vc_b), ClockRelation::Equal);

    // Increment process 0 on vc_a
    vc_a.increment(0);
    assert_eq!(vc_a.compare(&vc_b), ClockRelation::Greater);
    assert_eq!(vc_b.compare(&vc_a), ClockRelation::Less);

    // Increment process 1 on vc_b
    vc_b.increment(1);
    // Now they are concurrent (each has an event the other does not know about)
    assert_eq!(vc_a.compare(&vc_b), ClockRelation::Concurrent);
    assert_eq!(vc_b.compare(&vc_a), ClockRelation::Concurrent);

    // Merge vc_b into vc_a; after merge vc_a should dominate vc_b
    vc_a.merge(&vc_b);
    assert_eq!(vc_a.compare(&vc_b), ClockRelation::Greater);
    assert_eq!(vc_b.compare(&vc_a), ClockRelation::Less);

    // Increment process 2 on vc_b twice
    vc_b.increment(2);
    vc_b.increment(2);
    // Merge again, now both are equal
    vc_a.merge(&vc_b);
    vc_b.merge(&vc_a);
    assert_eq!(vc_a.compare(&vc_b), ClockRelation::Equal);
    assert_eq!(vc_b.compare(&vc_a), ClockRelation::Equal);

    // Edge case: large number of increments to test wrapping behaviour (using wrapping_add)
    let mut vc_c = VectorClock::new(1);
    for _ in 0..u64::MAX {
        vc_c.increment(0);
    }
    // At this point the counter is u64::MAX (since we started at 0 and added MAX times)
    assert_eq!(vc_c.counters[0], u64::MAX);
    // One more increment wraps to 0
    vc_c.increment(0);
    assert_eq!(vc_c.counters[0], 0);

    // Verify that merging with a zeroed clock does not change the clock
    let zero_clock = VectorClock::new(1);
    let before = vc_c.clone();
    vc_c.merge(&zero_clock);
    assert_eq!(vc_c, before);

    // Verify panic on size mismatch (uncomment to see panic at runtime)
    // let mut mismatched = VectorClock::new(2);
    // mismatched.increment(0);
    // vc_a.compare(&mismatched); // should panic

    // All assertions passed – the implementation behaves as expected.
    println!("All vector clock tests passed.");
}