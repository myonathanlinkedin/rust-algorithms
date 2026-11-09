mod core;

use core::BoundedBlockingQueue;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// Simple sequential sanity check.
fn basic_sequential_test() {
    let q = BoundedBlockingQueue::new(2);
    q.push(1);
    q.push(2);
    assert!(q.is_full());
    let a = q.pop();
    assert_eq!(a, 1);
    let b = q.pop();
    assert_eq!(b, 2);
    assert!(q.is_empty());
}

/// Stress test with multiple producers and a single consumer.
fn concurrent_test() {
    let capacity = 5;
    let q = Arc::new(BoundedBlockingQueue::new(capacity));
    let producers = 3usize;
    let items_per_producer = 20usize;
    let total_items = producers * items_per_producer;

    // Spawn producer threads.
    let mut prod_handles = Vec::with_capacity(producers);
    for p in 0..producers {
        let q_clone = Arc::clone(&q);
        prod_handles.push(thread::spawn(move || {
            for i in 0..items_per_producer {
                q_clone.push(p * items_per_producer + i);
                // Small pause to increase contention.
                thread::sleep(Duration::from_micros(10));
            }
        }));
    }

    // Consumer thread.
    let q_consumer = Arc::clone(&q);
    let consumer_handle = thread::spawn(move || {
        let mut collected = Vec::with_capacity(total_items);
        for _ in 0..total_items {
            let v = q_consumer.pop();
            collected.push(v);
        }
        collected
    });

    // Wait for all producers.
    for h in prod_handles {
        h.join().expect("producer panicked");
    }

    // Retrieve consumer results.
    let result = consumer_handle.join().expect("consumer panicked");
    assert_eq!(result.len(), total_items);

    // Verify that every expected value appears exactly once (order may differ).
    let mut sorted = result.clone();
    sorted.sort_unstable();
    let expected: Vec<_> = (0..total_items).collect();
    assert_eq!(sorted, expected);
}

fn main() {
    basic_sequential_test();
    concurrent_test();

    // Edge‑case checks for non‑blocking operations.
    let q = BoundedBlockingQueue::new(1);
    assert!(q.try_push(10).is_ok());
    assert!(q.try_push(20).is_err()); // queue is full
    assert_eq!(q.try_pop(), Some(10));
    assert_eq!(q.try_pop(), None); // now empty

    println!("All tests passed.");
}