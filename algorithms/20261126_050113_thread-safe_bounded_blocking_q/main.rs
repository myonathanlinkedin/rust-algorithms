mod core;
use core::BoundedQueue;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// Executes a series of self‑contained tests for `BoundedQueue`.
fn main() {
    test_basic_operations();
    test_try_operations();
    test_concurrent_producer_consumer();
    println!("All tests passed.");
}

/// Test push/pop behavior in a single thread.
fn test_basic_operations() {
    let q = BoundedQueue::new(3);
    assert!(q.is_empty());
    q.push(10);
    q.push(20);
    assert_eq!(q.len(), 2);
    assert_eq!(q.pop(), 10);
    assert_eq!(q.pop(), 20);
    assert!(q.is_empty());
}

/// Test non‑blocking `try_push` and `try_pop`.
fn test_try_operations() {
    let q = BoundedQueue::new(2);
    assert_eq!(q.try_pop(), None);
    assert_eq!(q.try_push(1), Ok(()));
    assert_eq!(q.try_push(2), Ok(()));
    // Queue is now full.
    assert_eq!(q.try_push(3), Err(3));
    assert_eq!(q.try_pop(), Some(1));
    assert_eq!(q.try_pop(), Some(2));
    assert_eq!(q.try_pop(), None);
}

/// Test that multiple producers and consumers correctly block and wake.
fn test_concurrent_producer_consumer() {
    let capacity = 2;
    let q = Arc::new(BoundedQueue::new(capacity));

    // Producer thread: pushes numbers 0..5 with a small delay.
    let producer = {
        let q = Arc::clone(&q);
        thread::spawn(move || {
            for i in 0..5 {
                q.push(i);
                // Small sleep to increase chance of contention.
                thread::sleep(Duration::from_millis(10));
            }
        })
    };

    // Consumer thread: pops five numbers and records them.
    let consumer = {
        let q = Arc::clone(&q);
        thread::spawn(move || {
            let mut collected = Vec::with_capacity(5);
            for _ in 0..5 {
                let val = q.pop();
                collected.push(val);
                thread::sleep(Duration::from_millis(15));
            }
            collected
        })
    };

    producer.join().expect("producer thread panicked");
    let result = consumer.join().expect("consumer thread panicked");

    // The queue preserves FIFO order.
    assert_eq!(result, vec![0, 1, 2, 3, 4]);

    // After all operations the queue must be empty.
    assert!(q.is_empty());
}