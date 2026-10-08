mod core;
use core::{AtomicCounter, SpinLock};
use std::sync::Arc;
use std::thread;

fn test_atomic_counter() {
    let counter = Arc::new(AtomicCounter::new(0));
    let mut handles = Vec::new();

    for _ in 0..8 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1_000 {
                c.increment();
            }
        }));
    }

    for h in handles {
        h.join().expect("thread panicked");
    }

    let final_val = counter.get();
    assert_eq!(final_val, 8 * 1_000);
    // Verify compare_and_set works
    let old = final_val;
    let success = counter.compare_and_set(old, 0);
    assert!(success);
    assert_eq!(counter.get(), 0);
}

fn test_spin_lock() {
    let lock = Arc::new(SpinLock::new(Vec::<usize>::new()));
    let mut handles = Vec::new();

    for i in 0..4 {
        let l = Arc::clone(&lock);
        handles.push(thread::spawn(move || {
            for j in 0..250 {
                let mut guard = l.lock();
                guard.push(i * 250 + j);
                // guard dropped here, releasing lock
            }
        }));
    }

    for h in handles {
        h.join().expect("thread panicked");
    }

    let guard = lock.lock();
    assert_eq!(guard.len(), 4 * 250);
    // Ensure all expected values are present
    let mut sorted = guard.clone();
    sorted.sort_unstable();
    for (idx, val) in sorted.iter().enumerate() {
        assert_eq!(*val, idx);
    }
}

fn main() {
    test_atomic_counter();
    test_spin_lock();
    println!("All concurrency tests passed.");
}