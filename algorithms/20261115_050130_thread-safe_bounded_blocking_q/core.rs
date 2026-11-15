use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};

#[derive(Debug)]
pub struct BoundedQueue<T> {
    pub inner: Mutex<Inner<T>>,
    pub not_empty: Condvar,
    pub not_full: Condvar,

}

#[derive(Debug)]
pub struct Inner<T> {
    pub queue: VecDeque<T>,
    pub capacity: usize,

}

impl<T> BoundedQueue<T> {
    /// Creates a new bounded queue with the given capacity.
    /// Panics if `capacity` is zero.
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must be greater than zero");
        BoundedQueue {
            inner: Mutex::new(Inner {
                queue: VecDeque::with_capacity(capacity),
                capacity,
            }),
            not_empty: Condvar::new(),
            not_full: Condvar::new(),
        }
    }

    /// Enqueues an element, blocking if the queue is full.
    /// Returns when the element has been inserted.
    pub fn push(&self, item: T) {
        let mut guard = self.inner.lock().unwrap();
        while guard.queue.len() == guard.capacity {
            guard = self.not_full.wait(guard).unwrap();
        }
        guard.queue.push_back(item);
        // Wake one consumer that might be waiting.
        self.not_empty.notify_one();
    }

    /// Dequeues an element, blocking if the queue is empty.
    /// Returns the removed element.
    pub fn pop(&self) -> T {
        let mut guard = self.inner.lock().unwrap();
        while guard.queue.is_empty() {
            guard = self.not_empty.wait(guard).unwrap();
        }
        let item = guard
            .queue
            .pop_front()
            .expect("queue was not empty after condition check");
        // Wake one producer that might be waiting.
        self.not_full.notify_one();
        item
    }

    /// Returns the current number of elements in the queue.
    /// This is a snapshot and may change immediately after the call.
    pub fn len(&self) -> usize {
        let guard = self.inner.lock().unwrap();
        guard.queue.len()
    }

    /// Returns the configured capacity of the queue.
    pub fn capacity(&self) -> usize {
        let guard = self.inner.lock().unwrap();
        guard.capacity
    }
}