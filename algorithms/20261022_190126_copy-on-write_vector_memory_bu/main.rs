mod core;
use core::CowVec;

fn main() {
    // Basic sanity checks executed at runtime.
    let mut a = CowVec::new();
    assert!(a.is_empty());
    a.push(10);
    a.push(20);
    assert_eq!(a.len(), 2);
    assert_eq!(a.get(0), Some(&10));
    assert_eq!(a.get(1), Some(&20));

    // Clone creates a shared buffer.
    let mut b = a.clone();
    assert_eq!(a.strong_count(), 2);
    assert_eq!(b.strong_count(), 2);
    assert_eq!(b.get(0), Some(&10));

    // Mutating `b` triggers copy‑on‑write.
    b.set(0, 99);
    assert_eq!(b.get(0), Some(&99));
    assert_eq!(a.get(0), Some(&10)); // `a` unchanged
    assert_eq!(a.strong_count(), 1);
    assert_eq!(b.strong_count(), 1);

    // Further mutations on `a` do not affect `b`.
    a.push(30);
    assert_eq!(a.len(), 3);
    assert_eq!(b.len(), 2);
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation_and_basic_ops() {
        let mut v = CowVec::from_slice(&[1, 2, 3]);
        assert_eq!(v.len(), 3);
        assert_eq!(v.get(1), Some(&2));

        v.set(1, 20);
        assert_eq!(v.get(1), Some(&20));

        v.push(40);
        assert_eq!(v.len(), 4);
        assert_eq!(v.get(3), Some(&40));
    }

    #[test]
    fn test_cow_behavior_on_set() {
        let mut original = CowVec::from_slice(&[5, 6, 7]);
        let mut clone = original.clone();

        // Both share the same buffer.
        assert_eq!(original.strong_count(), 2);
        assert_eq!(clone.strong_count(), 2);

        // Mutate clone; should detach.
        clone.set(0, 55);
        assert_eq!(clone.get(0), Some(&55));
        assert_eq!(original.get(0), Some(&5));

        // Reference counts after detachment.
        assert_eq!(original.strong_count(), 1);
        assert_eq!(clone.strong_count(), 1);
    }

    #[test]
    fn test_cow_behavior_on_push() {
        let mut v1 = CowVec::new();
        v1.push(100);
        let mut v2 = v1.clone();

        assert_eq!(v1.strong_count(), 2);
        assert_eq!(v2.strong_count(), 2);

        v2.push(200);
        assert_eq!(v2.len(), 2);
        assert_eq!(v1.len(), 1);
        assert_eq!(v1.get(0), Some(&100));
        assert_eq!(v2.get(1), Some(&200));

        assert_eq!(v1.strong_count(), 1);
        assert_eq!(v2.strong_count(), 1);
    }

    #[test]
    fn test_get_mut_triggers_cow() {
        let mut v1 = CowVec::from_slice(&[1, 2, 3]);
        let mut v2 = v1.clone();

        // Obtain mutable reference via get_mut on v2.
        if let Some(elem) = v2.get_mut(1) {
            *elem = 99;
        }

        // v1 must remain unchanged.
        assert_eq!(v1.get(1), Some(&2));
        assert_eq!(v2.get(1), Some(&99));

        // Ensure buffers are now distinct.
        assert_eq!(v1.strong_count(), 1);
        assert_eq!(v2.strong_count(), 1);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_set_out_of_bounds() {
        let mut v = CowVec::new();
        v.push(1);
        v.set(5, 10); // should panic
    }
}