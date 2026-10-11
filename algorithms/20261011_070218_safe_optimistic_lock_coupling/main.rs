mod core;
use core::OptimisticList;
use std::sync::{Arc, Barrier};
use std::thread;

fn sequential_tests() {
    let list = OptimisticList::new();

    assert!(!list.contains(10));
    assert!(list.insert(10));
    assert!(list.contains(10));
    assert!(!list.insert(10)); // duplicate
    assert!(list.insert(5));
    assert!(list.insert(15));
    assert_eq!(list.to_vec(), vec![5, 10, 15]);

    assert!(list.remove(10));
    assert!(!list.contains(10));
    assert!(!list.remove(10)); // already removed
    assert_eq!(list.to_vec(), vec![5, 15]);
}

fn concurrent_tests() {
    let list = Arc::new(OptimisticList::new());
    let thread_count = 8;
    let ops_per_thread = 1000;
    let barrier = Arc::new(Barrier::new(thread_count));

    let mut handles = Vec::new();

    for t in 0..thread_count {
        let list_cloned = Arc::clone(&list);
        let barrier_cloned = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            // Ensure all threads start together
            barrier_cloned.wait();
            for i in 0..ops_per_thread {
                let key = ((t * ops_per_thread) + i) as i32 % 50; // limited key space to force contention
                if i % 3 == 0 {
                    list_cloned.insert(key);
                } else if i % 3 == 1 {
                    list_cloned.remove(key);
                } else {
                    list_cloned.contains(key);
                }
            }
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().expect("thread panicked");
    }

    // Verify list is sorted and contains no duplicates
    let vec = list.to_vec();
    for w in vec.windows(2) {
        assert!(w[0] < w[1], "list not strictly sorted");
    }
    // All values must be within the expected key range
    for &v in &vec {
        assert!(v >= 0 && v < 50);
    }
}

fn main() {
    sequential_tests();
    concurrent_tests();
    // Simple sanity check
    let list = OptimisticList::new();
    assert!(list.insert(42));
    assert!(list.contains(42));
    assert!(list.remove(42));
    assert!(!list.contains(42));
}