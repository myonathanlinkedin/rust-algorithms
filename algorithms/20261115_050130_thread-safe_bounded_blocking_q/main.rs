mod core;
use core::BoundedQueue;
use std::sync::Arc;
use std::thread;

fn main() {
    // Simple demonstration with a single producer and consumer.
    let queue = Arc::new(BoundedQueue::new(3));
    let producer = {
        let q = Arc::clone(&queue);
        thread::spawn(move || {
            for i in 0..5 {
                q.push(i);
                // Optional: sleep to increase interleaving.
            }
        })
    };
    let consumer = {
        let q = Arc::clone(&queue);
        thread::spawn(move || {
            let mut collected = Vec::new();
            for _ in 0..5 {
                let v = q.pop();
                collected.push(v);
            }
            assert_eq!(collected, vec![0, 1, 2, 3, 4]);
        })
    };
    producer.join().expect("producer thread panicked");
    consumer.join().expect("consumer thread panicked");
    // After both threads finish, the queue must be empty.
    assert_eq!(queue.len(), 0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;

    #[test]
    fn test_single_thread_enqueue_dequeue() {
        let q = BoundedQueue::new(2);
        assert_eq!(q.len(), 0);
        q.push(10);
        assert_eq!(q.len(), 1);
        q.push(20);
        assert_eq!(q.len(), 2);
        assert_eq!(q.capacity(), 2);
        let a = q.pop();
        assert_eq!(a, 10);
        let b = q.pop();
        assert_eq!(b, 20);
        assert_eq!(q.len(), 0);
    }

    #[test]
    fn test_multi_thread_producers_consumers() {
        const PRODUCERS: usize = 4;
        const CONSUMERS: usize = 4;
        const ITEMS_PER_PRODUCER: usize = 2500;
        const CAPACITY: usize = 50;

        let queue = Arc::new(BoundedQueue::new(CAPACITY));
        let produced = Arc::new(AtomicUsize::new(0));
        let consumed = Arc::new(AtomicUsize::new(0));

        // Spawn producers.
        let mut prod_handles = Vec::new();
        for pid in 0..PRODUCERS {
            let q = Arc::clone(&queue);
            let prod_counter = Arc::clone(&produced);
            prod_handles.push(thread::spawn(move || {
                for i in 0..ITEMS_PER_PRODUCER {
                    q.push((pid, i));
                    prod_counter.fetch_add(1, Ordering::SeqCst);
                }
            }));
        }

        // Spawn consumers.
        let mut cons_handles = Vec::new();
        for _ in 0..CONSUMERS {
            let q = Arc::clone(&queue);
            let cons_counter = Arc::clone(&consumed);
            cons_handles.push(thread::spawn(move || {
                loop {
                    // Try to pop; break when all items have been consumed.
                    // We rely on the atomic counter to know when to stop.
                    let total_expected = PRODUCERS * ITEMS_PER_PRODUCER;
                    if cons_counter.load(Ordering::SeqCst) >= total_expected {
                        break;
                    }
                    let _ = q.pop();
                    cons_counter.fetch_add(1, Ordering::SeqCst);
                }
            }));
        }

        // Wait for all producers.
        for h in prod_handles {
            h.join().expect("producer panicked");
        }

        // Give consumers a moment to finish remaining items.
        while consumed.load(Ordering::SeqCst) < PRODUCERS * ITEMS_PER_PRODUCER {
            thread::sleep(Duration::from_millis(10));
        }

        // Wait for all consumers.
        for h in cons_handles {
            h.join().expect("consumer panicked");
        }

        assert_eq!(produced.load(Ordering::SeqCst), CONSUMERS * 0 + PRODUCERS * ITEMS_PER_PRODUCER);
        assert_eq!(consumed.load(Ordering::SeqCst), PRODUCERS * ITEMS_PER_PRODUCER);
        assert_eq!(queue.len(), 0);
    }

    #[test]
    #[should_panic(expected = "capacity must be greater than zero")]
    fn test_zero_capacity_panics() {
        let _ = BoundedQueue::<i32>::new(0);
    }

    #[test]
    fn test_capacity_respected() {
        let q = BoundedQueue::new(1);
        let q_arc = Arc::new(q);
        let q_clone = Arc::clone(&q_arc);
        let handle = thread::spawn(move || {
            // This push will fill the queue.
            q_clone.push(1);
            // This second push should block until a pop occurs.
            q_clone.push(2);
        });

        // Ensure the first element is in the queue.
        thread::sleep(Duration::from_millis(50));
        assert_eq!(q_arc.len(), 1);
        // Pop the first element, unblocking the producer.
        let v = q_arc.pop();
        assert_eq!(v, 1);
        // Now the second element should be available.
        let v2 = q_arc.pop();
        assert_eq!(v2, 2);
        handle.join().expect("producer thread panicked");
        assert_eq!(q_arc.len(), 0);
    }
}