mod core;
use core::BoundedQueue;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    // Simple sanity check executed at program start.
    let q = BoundedQueue::new(3);
    q.push(10);
    q.push(20);
    assert_eq!(q.pop(), 10);
    assert_eq!(q.pop(), 20);
    assert!(q.is_empty());
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::{self, Sender};

    #[test]
    fn test_fifo_behavior() {
        let q = BoundedQueue::new(5);
        q.push(1);
        q.push(2);
        q.push(3);
        assert_eq!(q.pop(), 1);
        assert_eq!(q.pop(), 2);
        assert_eq!(q.pop(), 3);
        assert!(q.is_empty());
    }

    #[test]
    fn test_blocking_pop() {
        let q = Arc::new(BoundedQueue::new(2));
        let q_clone = Arc::clone(&q);

        // Thread that will push after a short delay.
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));
            q_clone.push(42);
        });

        // This pop should block until the other thread pushes.
        let start = Instant::now();
        let value = q.pop();
        let elapsed = start.elapsed();

        assert_eq!(value, 42);
        // Ensure we actually waited at least 80ms (allowing some scheduler variance).
        assert!(elapsed >= Duration::from_millis(80));
        handle.join().unwrap();
    }

    #[test]
    fn test_blocking_push_when_full() {
        let q = Arc::new(BoundedQueue::new(2));
        q.push(1);
        q.push(2);

        let (tx, rx) = mpsc::channel::<()>();
        let q_producer = Arc::clone(&q);
        let producer_handle = thread::spawn(move || {
            // Signal that we are about to attempt a third push (which should block).
            tx.send(()).unwrap();
            q_producer.push(3); // This will block until a pop occurs.
        });

        // Ensure the producer thread has started and is blocked.
        rx.recv().unwrap();
        // Give the scheduler a moment to reach the blocking state.
        thread::sleep(Duration::from_millis(50));
        assert!(q.is_full());

        // Pop one element to free space.
        let popped = q.pop();
        assert_eq!(popped, 1);

        // Now the blocked push can proceed.
        producer_handle.join().unwrap();
        // Queue should contain the remaining original element and the newly pushed one.
        assert_eq!(q.pop(), 2);
        assert_eq!(q.pop(), 3);
        assert!(q.is_empty());
    }

    #[test]
    fn test_multiple_producers_consumers() {
        let q = Arc::new(BoundedQueue::new(3));
        let mut handles = Vec::new();

        // Producers
        for i in 0..3 {
            let q_prod = Arc::clone(&q);
            handles.push(thread::spawn(move || {
                q_prod.push(i);
            }));
        }

        // Consumers
        for _ in 0..3 {
            let q_cons = Arc::clone(&q);
            handles.push(thread::spawn(move || {
                let _ = q_cons.pop();
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        assert!(q.is_empty());
    }
}

// End of file.