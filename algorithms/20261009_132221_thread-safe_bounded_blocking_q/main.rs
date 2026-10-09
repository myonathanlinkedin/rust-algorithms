mod core;
use core::BoundedQueue;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// Simple sanity check executed at program start.
///
/// It pushes a few items and pops them back, asserting FIFO order.
fn basic_check() {
    let q = BoundedQueue::new(2);
    q.push(10);
    q.push(20);
    // The third push will block; spawn a thread to pop after a short delay.
    let q_clone = q.clone();
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        assert_eq!(q_clone.pop(), 10);
    });
    // This push should succeed after the consumer removes the first element.
    q.push(30);
    handle.join().unwrap();
    assert_eq!(q.pop(), 20);
    assert_eq!(q.pop(), 30);
    assert_eq!(q.len(), 0);
}

/// Stress test with multiple producers and consumers.
///
/// The test creates `n_producers` threads each pushing a distinct range of
/// numbers, and `n_consumers` threads each popping the same total amount.
/// It verifies that every produced value is received exactly once.
fn concurrent_stress_test() {
    const CAPACITY: usize = 5;
    const N_PRODUCERS: usize = 4;
    const N_CONSUMERS: usize = 4;
    const ITEMS_PER_PRODUCER: usize = 2500;

    let queue = Arc::new(BoundedQueue::new(CAPACITY));

    // Shared vector to record consumed values; protected by a mutex.
    let consumed = Arc::new(std::sync::Mutex::new(Vec::with_capacity(
        N_PRODUCERS * ITEMS_PER_PRODUCER,
    )));

    // Spawn producers.
    let mut prod_handles = Vec::new();
    for pid in 0..N_PRODUCERS {
        let q = Arc::clone(&queue);
        let start = pid * ITEMS_PER_PRODUCER;
        let end = start + ITEMS_PER_PRODUCER;
        prod_handles.push(thread::spawn(move || {
            for i in start..end {
                q.push(i);
            }
        }));
    }

    // Spawn consumers.
    let mut cons_handles = Vec::new();
    for _ in 0..N_CONSUMERS {
        let q = Arc::clone(&queue);
        let cons = Arc::clone(&consumed);
        cons_handles.push(thread::spawn(move || {
            loop {
                // Stop when we have collected all expected items.
                let mut guard = cons.lock().unwrap();
                if guard.len() >= N_PRODUCERS * ITEMS_PER_PRODUCER {
                    break;
                }
                drop(guard); // Release lock before blocking pop.
                let item = q.pop();
                let mut guard = cons.lock().unwrap();
                guard.push(item);
            }
        }));
    }

    // Wait for all producers to finish.
    for h in prod_handles {
        h.join().unwrap();
    }

    // Wait for consumers to finish.
    for h in cons_handles {
        h.join().unwrap();
    }

    // Verify that we received exactly the expected set of numbers.
    let mut result = consumed.lock().unwrap();
    result.sort_unstable();
    let expected: Vec<usize> = (0..N_PRODUCERS * ITEMS_PER_PRODUCER).collect();
    assert_eq!(&*result, &expected);
}

fn main() {
    basic_check();
    concurrent_stress_test();
    println!("All tests passed.");
}

// Unit tests (run with `cargo test`).
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Barrier;

    #[test]
    fn test_zero_capacity_panics() {
        let result = std::panic::catch_unwind(|| BoundedQueue::<i32>::new(0));
        assert!(result.is_err());
    }

    #[test]
    fn test_fifo_behavior() {
        let q = BoundedQueue::new(3);
        q.push(1);
        q.push(2);
        q.push(3);
        assert_eq!(q.pop(), 1);
        assert_eq!(q.pop(), 2);
        assert_eq!(q.pop(), 3);
    }

    #[test]
    fn test_blocking_push_and_pop() {
        let q = Arc::new(BoundedQueue::new(1));
        let barrier = Arc::new(Barrier::new(2));

        // Consumer thread: will pop after producer pushes.
        let q_c = Arc::clone(&q);
        let b_c = Arc::clone(&barrier);
        let consumer = thread::spawn(move || {
            b_c.wait(); // synchronize start
            assert_eq!(q_c.pop(), 42);
        });

        // Producer thread: pushes then waits.
        let q_p = Arc::clone(&q);
        let b_p = Arc::clone(&barrier);
        let producer = thread::spawn(move || {
            b_p.wait(); // synchronize start
            q_p.push(42);
        });

        producer.join().unwrap();
        consumer.join().unwrap();
    }

    #[test]
    fn test_multiple_producers_consumers() {
        // Reuse the stress test with smaller parameters for quick CI.
        const CAP: usize = 3;
        const PROD: usize = 2;
        const CONS: usize = 2;
        const ITEMS: usize = 500;

        let queue = Arc::new(BoundedQueue::new(CAP));
        let consumed = Arc::new(std::sync::Mutex::new(Vec::new()));

        // Producers
        let mut prod_handles = Vec::new();
        for pid in 0..PROD {
            let q = Arc::clone(&queue);
            prod_handles.push(thread::spawn(move || {
                for i in 0..ITEMS {
                    q.push(pid * ITEMS + i);
                }
            }));
        }

        // Consumers
        let mut cons_handles = Vec::new();
        for _ in 0..CONS {
            let q = Arc::clone(&queue);
            let cons = Arc::clone(&consumed);
            cons_handles.push(thread::spawn(move || {
                loop {
                    let mut guard = cons.lock().unwrap();
                    if guard.len() >= PROD * ITEMS {
                        break;
                    }
                    drop(guard);
                    let item = q.pop();
                    let mut guard = cons.lock().unwrap();
                    guard.push(item);
                }
            }));
        }

        for h in prod_handles {
            h.join().unwrap();
        }
        for h in cons_handles {
            h.join().unwrap();
        }

        let mut result = consumed.lock().unwrap();
        result.sort_unstable();
        let expected: Vec<usize> = (0..PROD * ITEMS).collect();
        assert_eq!(&*result, &expected);
    }
}